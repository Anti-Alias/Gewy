use glam::Vec2;

use crate::{Color, RawCorners, RawSides, RawMargin, RawPadding};
use bitflags::bitflags;

pub type Margin = Sides;
pub type Padding = Sides;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Style {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub margin: Sides,
    pub padding: Sides,
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
            top_right: corners.top_right.to_raw(parent_size),
            bottom_right: corners.bottom_right.to_raw(parent_size),
            bottom_left: corners.bottom_left.to_raw(parent_size)
        }
    }

    pub(crate) fn raw_margin(&self, parent_size: Vec2) -> RawMargin {
        Self::raw_sides(&self.margin, parent_size)
    }

    pub(crate) fn raw_padding(&self, parent_size: Vec2) -> RawPadding {
        Self::raw_sides(&self.padding, parent_size)
    }

    pub(crate) fn changes(&self, other: &Self) -> Changes {
        let mut changes = Changes::NONE;

        // Changes that affect everything
        if self.width != other.width || self.height != other.height || self.margin != other.margin || self.padding != other.padding || self.config != other.config {
            changes |= Changes::RECALCULATE;
            changes |= Changes::RECALCULATE_CHILDREN;
            changes |= Changes::REPAINT;
        }

        // Changes that only affect children
        if self.layout != other.layout {
            changes |= Changes::RECALCULATE_CHILDREN;
            changes |= Changes::REPAINT;
        }

        // Changes that only affect self (aesthetic)
        if self.color != other.color || self.corners != other.corners {
            changes |= Changes::REPAINT;
        }
        changes
    }

    fn raw_sides(sides: &Sides, parent_size: Vec2) -> RawSides {
        RawSides {
            top: sides.top.to_raw(parent_size.y),
            right: sides.right.to_raw(parent_size.x),
            bottom: sides.bottom.to_raw(parent_size.y),
            left: sides.left.to_raw(parent_size.x)
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
            Self::Pc(pc) => pc.clamp(0.0, 1.0) * parent,
            Self::Auto => parent
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sides {
    pub top: Val,
    pub right: Val,
    pub bottom: Val,
    pub left: Val
}

impl Default for Sides {
    fn default() -> Self {
        Self::all(Val::Px(0.0))
    }
}

impl Sides {
    pub fn new(top: Val, right: Val, bottom: Val, left: Val) -> Self {
        Self { top, right, bottom, left }
    }
    pub fn all(all: Val) -> Self {
        Self::new(all, all, all, all)
    }
    pub fn top(top: Val) -> Self {
        Self {
            top,
            ..Default::default()
        }
    }
    pub fn right(right: Val) -> Self {
        Self {
            right,
            ..Default::default()
        }
    }
    pub fn bottom(bottom: Val) -> Self {
        Self {
            bottom,
            ..Default::default()
        }
    }
    pub fn left(left: Val) -> Self {
        Self {
            left    ,
            ..Default::default()
        }
    }
}

/// Corner radiuses
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Corners {
    pub top_left: Val,
    pub top_right: Val,
    pub bottom_right: Val,
    pub bottom_left: Val
}

impl Default for Corners {
    fn default() -> Self {
        Self::all(Val::Px(0.0))
    }
}

impl Corners {
    pub fn new(top_left: Val, top_right: Val, bottom_right: Val, bottom_left: Val) -> Corners {
        Corners { top_left, top_right, bottom_right, bottom_left }
    }
    pub fn all(all: Val) -> Self {
        Corners::new(all, all, all, all)
    }
}

/// Flexbox layout for a container.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Layout {
    pub direction: Direction,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems
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

bitflags! {
    pub struct Changes: u8 {
        const NONE =                    0b00000000;
        const ALL =                     0b11111111;
        const REPAINT =                 0b00000001;
        const RECALCULATE_CHILDREN =    0b00000010;
        const RECALCULATE =             0b00000100;
    }
}