mod node;
mod widget;
mod pattern;
mod paint;
mod app;

pub use node::*;
pub use widget::*;
pub use pattern::*;
pub use paint::*;
pub use app::*;


#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b}
    }
}

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
