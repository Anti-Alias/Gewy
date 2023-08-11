use bytemuck::{Pod, Zeroable};
use wgpu::{*, util::{BufferInitDescriptor, DeviceExt}};
use crate::{Mat4, Vec3, Vec2};
use crate::wgpu::write_to_buffer;

/// Represents the "view" or "camera" that determines where polygons fall on the screen.
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Debug)]
pub struct View {
    pub proj_view: Mat4
}

impl View {

    pub fn new(size: Vec2, translation: Vec2, scale: f32) -> Self {
        let hw = size.x / 2.0;
        let hh = size.y as f32 / 2.0;
        let proj_view =
            Mat4::orthographic_rh(-hw, hw, -hh, hh, 0.0, 1.0) *
            Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0)) *
            Mat4::from_translation(translation.extend(0.0)) *
            Mat4::from_scale(Vec3::new(scale, scale, scale)) *
            Mat4::from_translation(Vec3::new(-hw, -hh, 0.0));
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
pub struct GpuView {
    pub buffer: Buffer,
    pub bind_group: BindGroup
}