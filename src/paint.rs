use wgpu::{*, util::{DeviceExt, BufferInitDescriptor}};
use crate::Color;
use glam::Vec2;
use std::mem::size_of;
use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Default, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4]
}

impl Vertex {

    const fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // Position (12 bytes)
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                },
                // Color (16 bytes)
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 12,
                    shader_location: 1
                }
            ]
        }
    }
}

/// A mesh of colored vertices.
#[derive(Clone, Default, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
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
            usage: BufferUsages::VERTEX,
        });
        let indices: &[u16] = &self.indices;
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Mesh Indices"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });
        GpuMesh { vertices, indices }
    }
}

pub struct GpuMesh {
    pub vertices: Buffer,
    pub indices: Buffer
}

/// A point in 2D space.
pub type Point = [f32; 2];

/// Painter that helps write vertices to a [`Mesh`] in a more structured and controlled way.
pub struct Painter<'m> {
    mesh: &'m mut Mesh,
    translation: Vec2,
    color: Color,
    index: u16
}

impl<'m> Painter<'m> {
    
    pub fn new(mesh: &'m mut Mesh) -> Self {
        Self {
            mesh,
            translation: Vec2::ZERO,
            color: Color::WHITE,
            index: 0
        }
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn triangle(&mut self, points: [Point; 3]) -> &mut Self {
        let i = self.index;
        self.mesh.vertices.extend(self.to_vertices(points));
        self.mesh.indices.extend([i+0, i+1, i+2]);
        self.index += 3;
        self
    }

    pub fn quad(&mut self, points: [Point; 4]) -> &mut Self {
        let i = self.index;
        self.mesh.vertices.extend(self.to_vertices(points));
        self.mesh.indices.extend([i+0, i+1, i+2, i+2, i+3, i+0]);
        self.index += 4;
        self
    }

    pub(crate) fn relative(self, translation: Vec2) -> Self {
        let mut result = Self::new(self.mesh);
        result.translation += translation;
        result
    }

    fn to_vertices<const N: usize>(&self, points: [Point; N]) -> [Vertex; N] {
        let t = self.translation;
        points.map(|p| {
            Vertex {
                position: [p[0] + t.x, p[1] + t.y],
                color: self.color.into()
            }
        })
    }
}

pub fn create_pipeline(device: &Device, texture_format: TextureFormat) -> RenderPipeline {
    let shader_source = include_str!("shader.wgsl");
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader"),
        source: ShaderSource::Wgsl(shader_source.into()),
    });
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        vertex: VertexState {
            module: &shader_module,
            entry_point: "vert_main",
            buffers: &[Vertex::layout()],
            //buffers: &[],
        },
        fragment: Some(FragmentState {
            module: &shader_module,
            entry_point: "frag_main",
            targets: &[Some(ColorTargetState {
                format: texture_format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL
            })]
        }),
        primitive: PrimitiveState::default(),
        multiview: None,
        layout: None,
        depth_stencil: None,
        multisample: MultisampleState::default()
    })
}
