use glam::{Vec2, Vec2Swizzles};

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
}