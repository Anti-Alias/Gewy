mod node;
mod widget;
mod pattern;
mod color;
mod paint;
mod app;
mod err;
mod math;
mod view;
mod util;

pub use node::*;
pub use widget::*;
pub use pattern::*;
pub use color::*;
pub use paint::*;
pub use app::*;
pub use math::*;
pub use err::*;
pub use view::*;
pub use util::*;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32
}

impl Margin {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32
}

impl Padding {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Style {
    pub color: Color,
    pub margin: Margin,
    pub padding: Padding
}
