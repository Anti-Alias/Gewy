use glam::Vec2;

use crate::{Color, RawCorners};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Style {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub margin: Margin,
    pub padding: Padding,
    pub corners: Corners,
    pub layout: Layout,
    pub config: Config
}

impl Style {
    pub(crate) fn raw_width(&self, parent_width: f32, is_row: bool) -> f32 {
        let width = if is_row { self.width } else { self.height };
        match width {
            Val::Px(px) => px,
            Val::Pc(pc) => pc.clamp(0.0, 1.0) * parent_width,
            Val::Auto => parent_width
        }
    }
    pub(crate) fn raw_height(&self, parent_height: f32, is_row: bool) -> f32 {
        let height = if is_row { self.height } else { self.width };
        match height {
            Val::Px(px) => px,
            Val::Pc(pc) => pc.clamp(0.0, 1.0) * parent_height,
            Val::Auto => parent_height
        }
    }
    pub(crate) fn raw_basis(&self, parent_width: f32, is_row: bool) -> f32 {
        match self.config.basis {
            Val::Px(px) => px,
            Val::Pc(pc) => pc.clamp(0.0, 1.0) * parent_width,
            Val::Auto => self.raw_width(parent_width, is_row)
        }
    }
    pub(crate) fn raw_corners(&self, parent_size: Vec2) -> RawCorners {
        let parent_size = parent_size.min_element();
        let corners = &self.corners;
        RawCorners {
            top_left: corners.top_left.to_raw(parent_size),
            top_right: corners.top_left.to_raw(parent_size),
            bottom_right: corners.top_left.to_raw(parent_size),
            bottom_left: corners.top_left.to_raw(parent_size)
        }
    }
}

/// Numerical value for various properties.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Val {
    /// Pixels
    Px(f32),
    /// Percent (0.0 - 1.0) of a parent's value.
    Pc(f32),
    /// Sourced from a value elsewhere.
    #[default]
    Auto
}

impl Val {
    pub fn to_raw(self, parent: f32) -> f32 {
        match self {
            Self::Px(px) => px,
            Self::Pc(pc) => parent * pc,
            Self::Auto => parent
        }
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
    pub fn scaled(self, scale: f32) -> Self {
        Self {
            top: self.top * scale,
            right: self.right * scale,
            bottom: self.bottom * scale,
            left: self.left * scale
        }
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
    pub fn scaled(self, scale: f32) -> Self {
        Self {
            top: self.top * scale,
            right: self.right * scale,
            bottom: self.bottom * scale,
            left: self.left * scale
        }
    }
}

/// Corner radiuses
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Corners {
    pub top_left: Val,
    pub top_right: Val,
    pub bottom_right: Val,
    pub bottom_left: Val
}

impl Corners {
    pub fn new(top_left: Val, top_right: Val, bottom_right: Val, bottom_left: Val) -> Corners {
        Corners { top_left, top_right, bottom_right, bottom_left }
    }
    pub fn all(all: Val) -> Self {
        Corners { top_left: all, top_right: all, bottom_right: all, bottom_left: all }
    }
}

/// Flexbox layout for a container.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Layout {
    pub direction: Direction,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent
}


/// Direction of a layout.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Direction {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse
}

impl Direction {
    pub fn is_reverse(self) -> bool {
        self == Self::RowReverse || self == Self::ColumnReverse
    }
    pub fn is_row(self) -> bool {
        self == Self::Row || self == Self::RowReverse
    }
}

/// Configuration for items in a [`Layout`].
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Config {
    pub grow: f32,
    pub shrink: f32,
    pub basis: Val,
    pub align_self: AlignSelf
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum JustifyContent {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum AlignItems {
    #[default]
    Center,
    Stretch,
    Start,
    End
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum AlignSelf {
    #[default]
    Auto,
    Stretch,
    Center,
    Start,
    End
}

impl AlignSelf {
    pub fn resolve(self, parent_align: AlignItems) -> AlignItems {
        match self {
            AlignSelf::Auto => parent_align,
            AlignSelf::Stretch => AlignItems::Stretch,
            AlignSelf::Center => AlignItems::Center,
            AlignSelf::Start => AlignItems::Start,
            AlignSelf::End => AlignItems::End
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum AlignContent {
    #[default]
    Stretch,
    Center,
    Start,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
}

/// Either an X or Y axis.
pub trait Axis {
    fn get_x<T>(x_val: T, y_val: T) -> T;
    fn get_y<T>(x_val: T, y_val: T) -> T;
    fn set_x<T>(x_val: &mut T, y_val: &mut T, val: T);
    fn set_y<T>(x_val: &mut T, y_val: &mut T, val: T);
}

pub struct XAxis;
impl Axis for XAxis {
    fn get_x<T>(x_val: T, _y_val: T) -> T { x_val }
    fn get_y<T>(_x_val: T, y_val: T) -> T { y_val }
    fn set_x<T>(x_val: &mut T, _y_val: &mut T, val: T) { *x_val = val }
    fn set_y<T>(_x_val: &mut T, y_val: &mut T, val: T) { *y_val = val }
}

pub struct YAxis;
impl Axis for YAxis {
    fn get_x<T>(_x_val: T, y_val: T) -> T { y_val }
    fn get_y<T>(x_val: T, _y_val: T) -> T { x_val }
    fn set_x<T>(_x_val: &mut T, y_val: &mut T, val: T) { *y_val = val }
    fn set_y<T>(x_val: &mut T, _y_val: &mut T, val: T) { *x_val = val }
}