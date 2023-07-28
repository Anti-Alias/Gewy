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

// Iterator over a slice that either travels forwards or backwards depending on a flag.
pub(crate) struct RevIter<'s, T> {
    index: isize,
    direction: isize,
    slice: &'s [T]
}

impl<'s, T: Copy> Iterator for RevIter<'s, T> {

    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let uindex = self.index as usize;
        if index < 0 || uindex == self.slice.len() {
            None
        }
        else {
            let result = unsafe { self.slice.get_unchecked(uindex)};
            self.index += self.direction;
            Some(result)
        }
    }
}

impl <'s, T> RevIter<'s, T> {
    pub fn new(slice: &'s [T], is_reversed: bool) -> Self {
        if is_reversed {
            Self {
                index: slice.len() as isize - 1,
                direction: -1,
                slice
            }
        }
        else {
            Self {
                index: 0,
                direction: 1,
                slice
            }
        }
    }
}