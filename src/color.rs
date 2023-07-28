use std::ops::{Add, Sub, Mul, Div};

use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Debug)]
pub struct Color { r: f32, g: f32, b: f32, a: f32 }
impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const DARK_GRAY: Color = Color::new(0.25, 0.25, 0.25, 1.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub const LIGHT_GRAY: Color = Color::new(0.75, 0.75, 0.75, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const LIGHT_BLUE: Color = Color::new(0.5, 0.5, 1.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const PINK: Color = Color::new(1.0, 0.0, 1.0, 1.0);
    pub const TEAL: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}
impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}
impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Self::new(value[0], value[1], value[2], 1.0)
    }
}

impl From<Color> for [f32; 4] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b, value.a]
    }
}
impl From<Color> for [f32; 3] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b]
    }
}
impl From<Color> for wgpu::Color {
    fn from(value: Color) -> Self {
        Self { r: value.r as f64, g: value.g as f64, b: value.b as f64, a: value.a as f64 }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b, self.a - rhs.a)
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a * rhs.a)
    }
}

impl Div for Color {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b, self.a / rhs.a)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs, self.a * rhs)
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.r / rhs, self.g / rhs, self.b / rhs, self.a / rhs)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output::new(self * rhs.r, self * rhs.g, self * rhs.b, self * rhs.a)
    }
}

impl Div<Color> for f32 {
    type Output = Color;
    fn div(self, rhs: Color) -> Self::Output {
        Self::Output::new(self / rhs.r, self / rhs.g, self / rhs.b, self / rhs.a)
    }
}