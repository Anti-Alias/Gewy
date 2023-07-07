use glam::{Vec2, Vec2Swizzles};

pub trait VecExtensions {
    fn flip(self, flip: bool) -> Vec2;
}

impl VecExtensions for Vec2 {
    fn flip(self, flip: bool) -> Vec2 {
        if flip { self.yx() } else { self }
    }
}