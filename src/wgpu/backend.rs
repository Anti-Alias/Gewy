use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use crate::Vec2;
use wgpu::*;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::DrawCommand;
use crate::wgpu::WGPUPainter;
use crate::{Color, wgpu::View};

pub struct WgpuBackend {
    render_pipeline: RenderPipeline,
    painter: WGPUPainter,
    view_format: TextureFormat,
    view_spp: u32,
    msaa_texture_view: Option<TextureView>
}

impl WgpuBackend {
   
    pub async fn new(
        device: &Device,
        view_format: TextureFormat,
        view_width: u32,
        view_height: u32,
        view_spp: u32,
        debug: bool,
    ) -> Self {
        let render_pipeline = create_pipeline(&device, view_format, view_spp, debug);
        let screen_size = Vec2::new(view_width as f32, view_height as f32);
        let painter = WGPUPainter::new(&device, screen_size);
        let msaa_texture = if view_spp != 0 {
            Some(create_msaa_texture_view(
                &device,
                view_format,
                view_width,
                view_height,
                view_spp)
            )
        }
        else {
            None
        };

        // Done
        Self {
            render_pipeline,
            painter,
            view_format,
            view_spp,
            msaa_texture_view: msaa_texture,
        }
    }

    pub fn resize(&mut self, view_width: u32, view_height: u32, device: &Device) {
        if self.view_spp != 1 {
            self.msaa_texture_view = Some(create_msaa_texture_view(
                device,
                self.view_format,
                view_width,
                view_height,
                self.view_spp
            ));
        }
    }

    pub fn render(
        &mut self,
        draw_commands: Vec<DrawCommand>,
        device: &Device,
        queue: &Queue,
        view: &TextureView
    ) -> Result<(), SurfaceError> {

        // Implements draw commands
        for command in draw_commands {
            self.painter.paint(command, device, queue);
        }
        self.painter.flush(device, queue);

        // Encodes render pass
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let color_attachment = if let Some(msaa_texture_view) = &self.msaa_texture_view {
            RenderPassColorAttachment {
                view: &msaa_texture_view,
                resolve_target: Some(&view),
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK.into()),
                    store: true
                },
            }
        }
        else {
            RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK.into()),
                    store: true
                },
            }
        };

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.painter.gpu_view.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.painter.gpu_mesh.vertices.slice(..));
        render_pass.set_index_buffer(self.painter.gpu_mesh.indices.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.painter.gpu_mesh.index_count, 0, 0..1);
        drop(render_pass);

        // Submits encoded draw calls
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

fn create_msaa_texture_view(
    device: &Device,
    view_format: TextureFormat,
    view_width: u32,
    view_height: u32,
    samples_per_pixel: u32
) -> TextureView {
    device
        .create_texture(&TextureDescriptor {
            label: Some("MSAA Texture"),
            size: Extent3d { width: view_width, height: view_height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: samples_per_pixel,
            dimension: TextureDimension::D2,
            format: view_format,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[view_format]
        })
        .create_view(&TextureViewDescriptor::default())
}

pub(crate) fn create_pipeline(
    device: &Device,
    view_format: TextureFormat,
    view_spp: u32,
    debug: bool
) -> RenderPipeline {
    let shader_source = include_str!("shader.wgsl");
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader"),
        source: ShaderSource::Wgsl(shader_source.into()),
    });
    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            &View::create_layout(device)
        ],
        push_constant_ranges: &[],
    });

    let polygon_mode = if debug { PolygonMode::Line } else { PolygonMode::Fill };
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        vertex: VertexState {
            module: &shader_module,
            entry_point: "vert_main",
            buffers: &[Vertex::layout()]
        },
        fragment: Some(FragmentState {
            module: &shader_module,
            entry_point: "frag_main",
            targets: &[Some(ColorTargetState {
                format: view_format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL
            })]
        }),
        primitive: PrimitiveState {
            polygon_mode,
            ..Default::default()
        },
        multiview: None,
        layout: Some(&layout),
        depth_stencil: None,
        multisample: MultisampleState {
            count: view_spp,
            ..Default::default()
        }
    })
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Default, Debug)]
pub struct Vertex {
    pub position: Vec2,
    pub color: Color
}

impl Vertex {
    pub fn new(position: Vec2, color: Color) -> Self {
        Self { position, color, }
    }
}

impl Vertex {

    const fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // Position (8 bytes)
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0
                },
                // Color (16 bytes)
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 8,
                    shader_location: 1
                }
            ]
        }
    }
}

/// A mesh of colored vertices.
#[derive(Clone, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

impl Mesh {

    pub fn new() -> Self {
        Self { vertices: Vec::new(), indices: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn to_gpu(&self, device: &Device) -> GpuMesh {
        let vertices: &[Vertex] = &self.vertices;
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Mesh Vertices"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST
        });
        let indices: &[u32] = &self.indices;
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Mesh Indices"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST
        });
        GpuMesh { vertices, indices, index_count: self.indices.len() as u32 }
    }

    pub fn write_to_gpu(&self, device: &Device, queue: &Queue, mesh: &mut GpuMesh) {
        let vertices: &[u8] = bytemuck::cast_slice(&self.vertices);
        let indices: &[u8] = bytemuck::cast_slice(&self.indices);
        write_to_buffer(&mut mesh.vertices, vertices, Some("Mesh Vertices"), device, queue);
        write_to_buffer(&mut mesh.indices, indices, Some("Mesh Indices"), device, queue);
        mesh.index_count = self.indices.len() as u32;
    }
}

pub struct GpuMesh {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub index_count: u32
}

/// A point in 2D space.
pub type Point = [f32; 2];

pub(crate) fn write_to_buffer(
    buffer: &mut Buffer,
    source: &[u8],
    label: Option<&str>,
    device: &Device,
    queue: &Queue
) {
    if source.len() as u64 > buffer.size() {
        *buffer = device.create_buffer(&BufferDescriptor {
            label,
            size: source.len() as u64,
            usage: buffer.usage(),
            mapped_at_creation: false,
        })
    }
    queue.write_buffer(&buffer, 0, source);
}