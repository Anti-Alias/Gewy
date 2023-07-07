use glam::{Vec2, Vec2Swizzles};

/// Simple rectangle struct.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    /// Top-left corner of the rectangle.
    pub pos: Vec2,
    /// Width and height of the rectangle.
    pub size: Vec2
}

impl Rect {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    pub fn flip(self, flip: bool) -> Self {
        if flip {
            Self::new(self.pos.yx(), self.size.yx())
        }
        else {
            self
        }
    }
}