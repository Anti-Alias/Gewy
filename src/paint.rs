use crate::Color;
use crate::Vec2;
use std::fmt::Debug;


/// Painter that writes primitive geometric shapes to a draw buffer.
/// Raw paint commands are to be implemented by a backend.
#[derive(Clone, Default, Debug)]
pub struct Painter {
    color: Color,
    translation: Vec2,
    pub(crate) commands: Vec<DrawCommand>
}

impl Painter {

    /// Creates empty painter.
    pub fn new() -> Self {
        Self {
            color: Color::WHITE,
            translation: Vec2::ZERO,
            commands: Vec::new()
        }
    }

    /// Moves future paint operations to the specified location.
    pub fn move_to(&mut self, location: Vec2) -> &mut Self {
        let global_translation = self.translation + location;
        self.commands.push(DrawCommand::Translation(global_translation));
        self
    }

    /// Sets the color.
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self.commands.push(DrawCommand::Color(color));
        self
    }

    /// Paints a circle with a radius.
    /// The number of points scales with the radius.
    pub fn paint_circle(&mut self, radius: f32) -> &mut Self {
        self.commands.push(DrawCommand::Circle { radius });
        self
    }

    /// Paints a rectangle.
    pub fn paint_rect(&mut self, size: Vec2) -> &mut Self {
        self.commands.push(DrawCommand::Rect { size });
        self
    }

    /// Paints a rectangle with rounded corners.
    pub fn paint_rounded_rect(&mut self, size: Vec2, top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> &mut Self{
        self.commands.push(DrawCommand::RoundedRect { size, top_left, top_right, bottom_right, bottom_left });
        self
    }

    pub(crate) fn set_translation(&mut self, translation: Vec2) -> &mut Self {
        self.translation = translation;
        self.commands.push(DrawCommand::Translation(translation));
        self
    }

    pub(crate) fn resize(&mut self, size: Vec2, translation: Vec2, scale: f32) {
        self.commands.push(DrawCommand::Resize { size, translation, scale })
    }

    pub(crate) fn push(&mut self) -> (Color, Vec2) {
        (self.color, self.translation)
    }

    pub(crate) fn pop(&mut self, state: (Color, Vec2)) {
        self.color = state.0;
        self.translation = state.1;
        self.commands.push(DrawCommand::Color(self.color));
        self.commands.push(DrawCommand::Translation(self.translation));
    }
}

/// Drawing primitive to be implemented via a rendering backend.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawCommand {
    Translation(Vec2),
    Color(Color),
    Circle { radius: f32 },
    Rect { size: Vec2 },
    RoundedRect { size: Vec2, top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32 },
    Resize { size: Vec2, translation: Vec2, scale: f32 }
}