use glam::Vec2;
use wgpu::*;

use crate::{Style, Painter, Canvas};

/// Utility function for painting pane-like widgets.
pub fn paint_pane(style: &Style, painter: &mut Painter, canvas: Canvas) {
    let old_color = painter.set_color(style.color);
    let Canvas { size, corners } = canvas;
    painter.rounded_rect(Vec2::ZERO, size, corners.top_left, corners.top_right, corners.bottom_right, corners.bottom_left);
    painter.color = old_color;
}

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