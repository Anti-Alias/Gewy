use wgpu::*;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::{Color, write_to_buffer, View};
use glam::Vec2;
use std::{mem::size_of, f32::consts::TAU, fmt::Debug};
use bytemuck::{Pod, Zeroable};


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

/// Painter that helps write vertices to a [`Mesh`] in a more structured and controlled way.
pub struct Painter {
    pub translation: Vec2,
    pub color: Color,
    mesh: Mesh,
    index: u32
}

impl Painter {
    
    pub(crate) fn new(mesh: Mesh) -> Self {
        Self {
            translation: Vec2::ZERO,
            color: Color::WHITE,
            mesh,
            index: 0
        }
    }

    /// Paints a triangle.
    pub fn triangle(&mut self, points: [Vec2; 3]) -> &mut Self {
        let i = self.index;
        self.mesh.vertices.extend(self.to_vertices(points));
        self.mesh.indices.extend([i+0, i+1, i+2]);
        self.index += 3;
        self
    }

    /// Paints a rectangle.
    pub fn rect(&mut self, position: Vec2, size: Vec2) -> &mut Self {
        let p = position;
        self.quad([
            Vec2::new(p.x, p.y),
            Vec2::new(p.x + size.x, p.y),
            Vec2::new(p.x + size.x, p.y + size.y),
            Vec2::new(p.x, p.y + size.y)
        ]);
        self
    }

    /// Paints a circle with a radius.
    /// The number of points scales with the radius.
    pub fn circle(&mut self, center: Vec2, radius: f32) -> &mut Self {

        let num_verts = radius_to_vertex_count(radius);
        if num_verts < 3 { return self }
        let num_indices = num_verts * 3 - 6;

        self.mesh.vertices.reserve(num_verts as usize);
        self.mesh.indices.reserve(num_indices as usize);
        
        // Writes vertices
        for i in 0..num_verts {
            let radians = TAU * i as f32 / num_verts as f32;
            let position = Vec2::from_angle(radians) * radius + center;
            self.mesh.vertices.push(Vertex {
                position: position + self.translation,
                color: self.color,
            });
        }

        // Writes indices
        for i in 1..num_verts-1 {
            self.mesh.indices.push(self.index);
            self.mesh.indices.push(self.index + i);
            self.mesh.indices.push(self.index + i + 1);
        }
        self.index += num_verts;
        self
    }

    /// Paint a quad with four points.
    pub fn quad(&mut self, points: [Vec2; 4]) -> &mut Self {
        let i = self.index;
        self.mesh.vertices.extend(self.to_vertices(points));
        self.mesh.indices.extend([i+0, i+1, i+2, i+2, i+3, i+0]);
        self.index += 4;
        self
    }

    /// Creates a shape painter that references this painter.
    pub fn shape(&mut self) -> ShapePainter<'_> {
        ShapePainter { painter: self }
    }

    pub(crate) fn flush(&mut self, device: &Device, queue: &Queue, gpu_mesh: &mut GpuMesh) {
        self.mesh.write_to_gpu(device, queue, gpu_mesh);
        self.mesh.clear();
        self.index = 0;
    }

    fn to_vertices<const N: usize>(&self, points: [Vec2; N]) -> [Vertex; N] {
        points.map(|point| Vertex::new(point + self.translation, self.color))
    }
}

/// Paints triangles as a "fan" (https://www.khronos.org/opengl/wiki/Primitive).
pub struct ShapePainter<'p> {
    painter: &'p mut Painter
}

impl<'p> ShapePainter<'p> {
    pub fn vertex(&mut self, mut v: Vertex) {
        v.position += self.painter.translation;
        self.painter.mesh.vertices.push(v);
    }
    pub fn vertices<const N: usize>(&mut self, vertices: [Vertex; N]) {
        let t = self.painter.translation;
        let vertices = vertices.map(|mut v| { v.position += t; v });
        self.painter.mesh.vertices.extend(vertices);
    }
    pub fn point(&mut self, point: Vec2) {
        self.vertex(Vertex { position: point, color: self.painter.color });
    }
    pub fn points<const N: usize>(&mut self, points: [Vec2; N]) {
        let vertices = points.map(|point| Vertex { position: point, color: self.painter.color });
        self.vertices(vertices);
    }
    pub fn quarter_circle(&mut self, center: Vec2, radius: f32, radians_offset: f32) -> &mut Self {
        let circle_vertex_count = radius_to_vertex_count(radius);
        if circle_vertex_count < 3 {
            self.point(center);
            return self;
        }
        let vertex_count = circle_vertex_count / 4 + 1;
        let circle_vertex_count = circle_vertex_count as f32;
        for i in 0..vertex_count {
            let i = i as f32;
            let radians = TAU * i / circle_vertex_count;
            let point = center + Vec2::from_angle(radians + radians_offset) * radius;
            self.point(point);
        }
        self
    }
}

impl<'p> Drop for ShapePainter<'p> {
    fn drop(&mut self) {

        // Determines the number of vertices already written
        let mesh = &mut self.painter.mesh;
        let vertices_added = {
            let vertex_count = mesh.vertices.len() as u32;
            let vertices_added = vertex_count - self.painter.index;
            if vertices_added == 0 { return }
            if vertices_added < 3 {
                panic!("ConvexPainter wrote {vertices_added} vertices.");
            }
            vertices_added
        };

        // Writes indices
        let indices_to_add = vertices_added * 3 - 6;
        mesh.indices.reserve(indices_to_add as usize);
        for i in 1..vertices_added-1 {
            mesh.indices.push(self.painter.index);
            mesh.indices.push(self.painter.index + i);
            mesh.indices.push(self.painter.index + i + 1);
        }
        self.painter.index += vertices_added;
    }
}

pub(crate) fn create_pipeline(device: &Device, texture_format: TextureFormat, debug: bool) -> RenderPipeline {
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
                format: texture_format,
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
        multisample: MultisampleState::default()
    })
}


fn radius_to_vertex_count(radius: f32) -> u32 {
    radius as u32
}