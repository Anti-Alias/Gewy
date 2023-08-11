use wgpu::*;
use crate::{Color, DrawCommand};
use crate::wgpu::{Mesh, GpuMesh, View, GpuView, Vertex};
use glam::Vec2;
use std::f32::consts::{FRAC_PI_2, PI};
use std::f32::consts::TAU;

/// WGPU-backend for [`crate::Painter`]
pub struct WGPUPainter {
    pub color: Color,
    pub translation: Vec2,
    pub(crate) gpu_mesh: GpuMesh,
    pub(crate) gpu_view: GpuView,
    mesh: Mesh,
    view: View,
    index: u32,
    polygon_scale: f32
}

impl WGPUPainter {
    
    pub fn new(device: &Device, screen_size: Vec2) -> Self {
        let mesh = Mesh::new();
        let gpu_mesh = mesh.to_gpu(device);
        let view = View::new(screen_size, Vec2::ZERO, 1.0);
        let gpu_view = view.to_gpu(&device);
        Self {
            translation: Vec2::ZERO,
            color: Color::WHITE,
            mesh,
            gpu_mesh,
            view,
            gpu_view,
            index: 0,
            polygon_scale: 1.0
        }
    }

    pub fn paint(&mut self, command: DrawCommand, device: &Device, queue: &Queue) {
        match command {
            DrawCommand::Translation(translation) => self.translation = translation,
            DrawCommand::Color(color) => self.color = color,
            DrawCommand::Circle { radius } => self.circle(radius),
            DrawCommand::Rect { size } => self.rect(size),
            DrawCommand::RoundedRect { size, top_left, top_right, bottom_right, bottom_left } => self.rounded_rect(size, top_left, top_right, bottom_right, bottom_left),
            DrawCommand::Resize { size, translation, scale } => self.resize(size, translation, scale, device, queue)
        }
    }

    /// Paints a triangle.
    pub fn triangle(&mut self, points: [Vec2; 3]) {
        let i = self.index;
        self.mesh.vertices.extend(self.points_to_vertices(points));
        self.mesh.indices.extend([i+0, i+1, i+2]);
        self.index += 3;
    }

    /// Paints a rectangle.
    pub fn rect(&mut self, size: Vec2) {
        let p = self.translation;
        self.quad([
            Vec2::new(0.0, 0.0),
            Vec2::new(size.x, p.y),
            Vec2::new(size.x, size.y),
            Vec2::new(0.0, p.y + size.y)
        ]);
    }

    /// Paints a rectangle with rounded corners.
    pub fn rounded_rect(&mut self, size: Vec2, top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) {
       
        // Caps border radiuses.
        let max_radius = (size / 2.0).min_element();
        let top_left = top_left.min(max_radius);
        let top_right = top_right.min(max_radius);
        let bottom_right = bottom_right.min(max_radius);
        let bottom_left = bottom_left.min(max_radius);

        // Calculates centers of border radiuses.
        let c_tl = Vec2::new(top_left, top_left);
        let c_tr = Vec2::new(size.x - top_right, top_right);
        let c_br = Vec2::new(size.x - bottom_right, size.y - bottom_right);
        let c_bl = Vec2::new(bottom_left, size.y - bottom_left);
        
        // Paints borders.
        let mut shape = self.shape();
        shape.quarter_circle(c_tl, top_left, PI);
        shape.quarter_circle(c_tr, top_right, 3.0 * FRAC_PI_2);
        shape.quarter_circle(c_br, bottom_right, 0.0);
        shape.quarter_circle(c_bl, bottom_left, FRAC_PI_2);
        drop(shape);
    }

    /// Paints a circle with a radius.
    /// The number of points scales with the radius.
    pub fn circle(&mut self, radius: f32) {

        // Calculate the number of vertices and indices
        let num_verts = radius_to_vertex_count(radius * self.polygon_scale);
        if num_verts < 3 { return; }
        let num_indices = num_verts * 3 - 6;
        self.mesh.vertices.reserve(num_verts as usize);
        self.mesh.indices.reserve(num_indices as usize);
        
        // Writes vertices
        for i in 0..num_verts {
            let radians = TAU * i as f32 / num_verts as f32;
            let position = Vec2::from_angle(radians) * radius;
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
    }

    pub(crate) fn quad(&mut self, points: [Vec2; 4]) {
        let i = self.index;
        self.mesh.vertices.extend(self.points_to_vertices(points));
        self.mesh.indices.extend([i+0, i+1, i+2, i+2, i+3, i+0]);
        self.index += 4;
    }

    pub(crate) fn shape(&mut self) -> ShapePainter<'_> {
        ShapePainter { painter: self }
    }

    pub(crate) fn flush(&mut self, device: &Device, queue: &Queue) {
        self.mesh.write_to_gpu(device, queue, &mut self.gpu_mesh);
        self.mesh.clear();
        self.index = 0;
    }

    pub(crate) fn resize(&mut self, size: Vec2, translation: Vec2, scale: f32, device: &Device, queue: &Queue) {
        self.view = View::new(size, translation, scale);
        self.polygon_scale = scale;
        self.view.write_to_gpu(device, queue, &mut self.gpu_view)
    }

    // Translates points and turns them into vertices.
    fn points_to_vertices<const N: usize>(&self, points: [Vec2; N]) -> [Vertex; N] {
        points.map(|point| Vertex::new(point + self.translation, self.color))
    }
}

/// Paints triangles as a "fan" (https://www.khronos.org/opengl/wiki/Primitive).
pub(crate) struct ShapePainter<'p> {
    painter: &'p mut WGPUPainter
}

impl<'p> ShapePainter<'p> {
    pub fn vertex(&mut self, mut v: Vertex) {
        v.position += self.painter.translation;
        self.painter.mesh.vertices.push(v);
    }
    pub fn point(&mut self, point: Vec2) {
        self.vertex(Vertex { position: point, color: self.painter.color });
    }
    pub fn quarter_circle(&mut self, center: Vec2, radius: f32, radians_offset: f32) {
        
        // Push
        let translation = self.painter.translation;
        self.painter.translation += center;

        // Draw
        let vertex_count = radius_to_quarter_vertex_count(radius * self.painter.polygon_scale);
        if vertex_count < 2 {
            self.point(Vec2::ZERO);
        }
        else {
            let divisor = (vertex_count - 1) as f32;
            for i in 0..vertex_count {
                let i = i as f32;
                let ratio = i as f32 / divisor;
                let radians = FRAC_PI_2 * ratio;
                self.point(Vec2::from_angle(radians + radians_offset) * radius);
            }
        }

        // Pop
        self.painter.translation = translation;
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


fn radius_to_vertex_count(radius: f32) -> u32 {
    let scaled = radius.powf(0.8) * 2.0;
    scaled as u32 + 8
}

fn radius_to_quarter_vertex_count(radius: f32) -> u32 {
    let scaled = radius.powf(0.8);
    (scaled / 4.0 ) as u32 + 8
}