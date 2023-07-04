use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use wgpu::{*, util::{BufferInitDescriptor, DeviceExt}};
use winit::dpi::PhysicalSize;

use crate::write_to_buffer;

/// Represents the "view" or "camera" that determines where polygons fall on the screen.
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Debug)]
pub(crate) struct View {
    pub proj_view: Mat4
}

impl View {

    pub fn from_physical_size(size: PhysicalSize<u32>) -> Self {
        let hw = size.width as f32 / 2.0;
        let hh = size.height as f32 / 2.0;
        let proj_view =
            Mat4::orthographic_rh(-hw, hw, -hh, hh, 0.0, 1.0) *
            Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0)) *
            Mat4::from_translation(Vec3::new(-hw, -hh, 0.0));
        //let proj_view = 2.0 * Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));
        Self { proj_view }
    }

    pub fn create_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("View Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }]
        })
    }

    pub fn to_gpu(&self, device: &Device) -> GpuView {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("View"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        let buffer_binding = BufferBinding { buffer: &buffer, offset: 0, size: None };
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("View"),
            layout: &Self::create_layout(device),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(buffer_binding)
            }]
        });
        GpuView { buffer, bind_group }
    }

    pub fn write_to_gpu(&self, device: &Device, queue: &Queue, view: &mut GpuView) {
        write_to_buffer(&mut view.buffer, bytemuck::cast_slice(&[*self]), Some("View"), device, queue);
    }
}

/// GPU representation of a [`View`] as a bind group.
pub(crate) struct GpuView {
    pub buffer: Buffer,
    pub bind_group: BindGroup
}