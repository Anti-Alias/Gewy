use glam::Vec2;

use crate::{Color, RawCorners, RawSides, RawMargin, RawPadding};

pub type Margin = Sides;
pub type Padding = Sides;

#[derive(Clone, PartialEq, Debug)]
pub struct Style {
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,
    pub color: Color,
    pub margin: Sides,
    pub padding: Sides,
    pub corners: Corners,
    pub direction: Direction,
    pub justify: Justify,
    pub align: Align,
    pub grow: f32,
    pub shrink: f32,
    pub basis: Val,
    pub align_self: AlignSelf
}

impl Style {
    pub(crate) fn raw_width(&self, parent_width: f32, is_row: bool) -> f32 {
        let width = if is_row { self.width } else { self.height };
        match width {
            Val::Px(px) => px.max(0.0),
            Val::Pc(pc) => pc.clamp(0.0, 1.0) * parent_width,
            Val::Auto => parent_width
        }
    }
    pub(crate) fn raw_height(&self, parent_height: f32, is_row: bool) -> f32 {
        let height = if is_row { self.height } else { self.width };
        match height {
            Val::Px(px) => px.max(0.0),
            Val::Pc(pc) => pc.clamp(0.0, 1.0) * parent_height,
            Val::Auto => parent_height
        }
    }
    pub(crate) fn raw_basis(&self, parent_width: f32, is_row: bool) -> f32 {
        match self.basis {
            Val::Px(px) => px.max(0.0),
            Val::Pc(pc) => pc.clamp(0.0, 1.0) * parent_width,
            Val::Auto => self.raw_width(parent_width, is_row)
        }
    }
    pub(crate) fn raw_corners(&self, element_size: Vec2) -> RawCorners {
        let parent_size = element_size.min_element();
        let corners = &self.corners;
        RawCorners {
            top_left: corners.top_left.to_raw(parent_size),
            top_right: corners.top_right.to_raw(parent_size),
            bottom_right: corners.bottom_right.to_raw(parent_size),
            bottom_left: corners.bottom_left.to_raw(parent_size)
        }
    }

    pub(crate) fn raw_margin(&self, parent_size: Vec2, is_row: bool) -> RawMargin {
        Self::raw_sides(&self.margin, parent_size, is_row)
    }

    pub(crate) fn raw_padding(&self, parent_size: Vec2, is_row: bool) -> RawPadding {
        Self::raw_sides(&self.padding, parent_size, is_row)
    }

    pub(crate) fn raw_min_size(&self, parent_size: Vec2, is_row: bool) -> Vec2 {
        let (width, height) = if is_row {
            (self.min_width, self.min_height)
        }
        else {
            (self.min_height, self.min_width)
        };
        Vec2::new(width.to_raw(parent_size.x), height.to_raw(parent_size.y))
    }

    pub(crate) fn raw_max_size(&self, parent_size: Vec2, is_row: bool) -> Vec2 {
        let (width, height) = if is_row {
            (self.max_width, self.max_height)
        }
        else {
            (self.max_height, self.max_width)
        };
        Vec2::new(width.to_raw(parent_size.x), height.to_raw(parent_size.y))
    }

    fn raw_sides(sides: &Sides, parent_size: Vec2, is_row: bool) -> RawSides {
        RawSides {
            top: sides.top.to_raw(parent_size.y).max(0.0),
            right: sides.right.to_raw(parent_size.x).max(0.0),
            bottom: sides.bottom.to_raw(parent_size.y).max(0.0),
            left: sides.left.to_raw(parent_size.x).max(0.0)
        }.rotate_right(!is_row)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: Val::default(),
            height: Val::default(),
            min_width: Val::Px(0.0),
            min_height: Val::Px(0.0),
            max_width: Val::default(),
            max_height: Val::default(),
            color: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            corners: Default::default(),
            justify: Default::default(),
            direction: Default::default(),
            align: Default::default(),
            grow: 0.0,
            shrink: 1.0,
            basis: Val::default(),
            align_self: AlignSelf::default()
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

/// The layout of a node.
/// Determines how children are "laid out".
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Layout {
    pub direction: Direction,
    pub justify: Justify,
    pub align: Align
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

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Justify {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Align {
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
    pub fn to_align_items(self, auto_value: Align) -> Align {
        match self {
            AlignSelf::Auto => auto_value,
            AlignSelf::Stretch => Align::Stretch,
            AlignSelf::Center => Align::Center,
            AlignSelf::Start => Align::Start,
            AlignSelf::End => Align::End
        }
    }
}