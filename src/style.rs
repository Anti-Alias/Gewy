use crate::Color;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Style {
    pub width: Val,
    pub height: Val,
    pub corners: Corners,
    pub color: Color,
    pub margin: Margin,
    pub padding: Padding,
    pub layout: Layout,
    pub config: Config
}

impl Style {
    pub fn get_width(&self, parent_width: f32, is_row: bool) -> f32 {
        let width = if is_row { self.width } else { self.height };
        match width {
            Val::Px(px) => px,
            Val::Pc(pc) => pc * parent_width,
            Val::Auto => parent_width
        }
    }
    pub fn get_height(&self, parent_height: f32, is_row: bool) -> f32 {
        let height = if is_row { self.height } else { self.width };
        match height {
            Val::Px(px) => px,
            Val::Pc(pc) => pc * parent_height,
            Val::Auto => parent_height
        }
    }
    pub fn get_basis(&self, parent_width: f32, is_row: bool) -> f32 {
        match self.config.basis {
            Val::Px(px) => px,
            Val::Pc(pc) => parent_width * pc,
            Val::Auto => self.get_width(parent_width, is_row)
        }
    }
}

/// Numerical value for various properties.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Val {
    /// Pixels
    Px(f32),
    /// Percent (0.0 - 1.0) of a value sourced elsewhere.
    Pc(f32),
    /// Sourced from a value elsewhere.
    #[default]
    Auto
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

/// Corner radiuses
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Corners {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32
}

impl Corners {
    pub fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Corners {
        Corners { top_left, top_right, bottom_right, bottom_left }
    }
    pub fn all(all: f32) -> Self {
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
    pub basis: Val
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
    Stretch,
    Center,
    Start,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
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