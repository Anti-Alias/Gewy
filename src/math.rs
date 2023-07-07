use glam::Vec2;

use crate::{extensions::VecExtensions, Axis};

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
    pub fn get_x<A: Axis>(&self) -> f32 { self.pos.get_x::<A>() }
    pub fn get_y<A: Axis>(&self) -> f32 { self.pos.get_y::<A>() }
    pub fn get_pos<A: Axis>(&self) -> Vec2 { self.pos.get_xy::<A>() }
    pub fn set_x<A: Axis>(&mut self, x: f32) { self.pos.set_x::<A>(x); }
    pub fn set_y<A: Axis>(&mut self, y: f32) { self.pos.set_y::<A>(y); }
    pub fn set_pos<A: Axis>(&mut self, xy: Vec2) { self.pos.set_xy::<A>(xy); }
    pub fn get_width<A: Axis>(&self) -> f32 { self.size.get_x::<A>() }
    pub fn get_height<A: Axis>(&self) -> f32 { self.size.get_y::<A>() }
    pub fn get_size<A: Axis>(&self) -> Vec2 { self.size.get_xy::<A>() }
    pub fn set_width<A: Axis>(&mut self, width: f32) { self.size.set_x::<A>(width); }
    pub fn set_height<A: Axis>(&mut self, y: f32) { self.size.set_y::<A>(y); }
    pub fn set_size<A: Axis>(&mut self, xy: Vec2) { self.size.set_xy::<A>(xy); }
}