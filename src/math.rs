use glam::{Vec2, Vec2Swizzles};

use crate::extensions::VecExtensions;

/// Simple rectangle struct.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    /// Top-left corner of the rectangle.
    pub position: Vec2,
    /// Width and height of the rectangle.
    pub size: Vec2
}

impl Rect {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { position: pos, size }
    }

    pub fn flip(self, flip: bool) -> Self {
        if flip {
            Self::new(self.position.yx(), self.size.yx())
        }
        else {
            self
        }
    }

    pub fn round(self, unit: f32) -> Self {
        Self {
            position: (self.position / unit).round() * unit,
            size: (self.size / unit).round() * unit
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        let tl = self.position;
        let br = self.position + self.size;
        point.x >= tl.x &&
        point.y >= tl.y &&
        point.x <= br.x &&
        point.y <= br.y
    }

    pub fn non_negative(&self) -> Self {
        Self {
            position: self.position.non_negative(),
            size: self.size.non_negative()
        }
    }
}