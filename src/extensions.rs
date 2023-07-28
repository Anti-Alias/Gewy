use glam::{Vec2, Vec2Swizzles};

pub trait VecExtensions {
    fn flip(self, flip: bool) -> Vec2;
    fn non_negative(self) -> Self;
}

impl VecExtensions for Vec2 {
    fn flip(self, flip: bool) -> Vec2 {
        if flip {
            self.yx()
        }
        else {
            self
        }
    }
    fn non_negative(self) -> Self {
        Vec2::new(self.x.max(0.0), self.y.max(0.0))
    }
}