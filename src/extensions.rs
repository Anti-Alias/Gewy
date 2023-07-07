use glam::Vec2;

use crate::Axis;

pub trait VecExtensions {
    fn newa<A: Axis>(x: f32, y: f32) -> Self;
    fn flip<A: Axis>(self) -> Self;
    fn get_x<A: Axis>(self) -> f32;
    fn get_y<A: Axis>(self) -> f32;
    fn get_xy<A: Axis>(self) -> Self;
    fn set_x<A: Axis>(&mut self, x: f32);
    fn set_y<A: Axis>(&mut self, y: f32);
    fn set_xy<A: Axis>(&mut self, xy: Vec2);
}

impl VecExtensions for Vec2 {
    fn newa<A: Axis>(x: f32, y: f32) -> Self {
        Self::new(A::get_x(x, y), A::get_y(x, y))
    }
    fn flip<A: Axis>(self) -> Self {
        Self {
            x: A::get_x(self.x, self.y),
            y: A::get_y(self.x, self.y)
        }
    }
    fn get_x<A: Axis>(self) -> f32 { A::get_x(self.x, self.y) }
    fn get_y<A: Axis>(self) -> f32 { A::get_y(self.x, self.y) }
    fn get_xy<A: Axis>(self) -> Self {
        Self::new(
            A::get_x(self.x, self.y),
            A::get_y(self.x, self.y)
        )
    }

    fn set_x<A: Axis>(&mut self, x: f32) {
        A::set_x(&mut self.x, &mut self.y, x)
    }

    fn set_y<A: Axis>(&mut self, y: f32) {
        A::set_y(&mut self.x, &mut self.y, y)
    }

    fn set_xy<A: Axis>(&mut self, xy: Vec2) {
        A::set_x(&mut self.x, &mut self.y, xy.x);
        A::set_y(&mut self.x, &mut self.y, xy.y);
    }
    }