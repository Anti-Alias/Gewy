use glam::Vec2;

use crate::Rect;

pub type RawMargin = RawSides;
pub type RawPadding = RawSides;

/// Stores raw values computed during the layout phase.
/// Units of all values are stored in pixels.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub(crate) struct Raw {
    pub region: Rect,
    pub margin: RawMargin,
    pub padding: RawPadding,
    pub corners: RawCorners
}

impl Raw {

    // Region with content + padding (no margin)
    pub fn outer_region(&self) -> Rect {
        let margin = &self.margin;
        let top_left = Vec2::new(margin.left, margin.top);
        let bottom_right = Vec2::new(margin.right, margin.bottom);
        let position = self.region.position + top_left;
        let size = self.region.size - top_left - bottom_right;
        Rect { position, size }
    }

    // Region where children reside (no margin, no padding).
    pub fn inner_region(&self) -> Rect {
        let (margin, padding) = (&self.margin, &self.padding);
        let top_left = Vec2::new(margin.left, margin.top) + Vec2::new(padding.left, padding.top);
        let bottom_right = Vec2::new(margin.right, margin.bottom) + Vec2::new(padding.right, padding.bottom);
        let position = self.region.position + top_left;
        let size = self.region.size - top_left - bottom_right;
        Rect { position, size }
    }
}

/// Raw variant of [`crate::Corners`]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct RawCorners {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct RawSides {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32
}

impl RawSides {
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.right() + self.left(), self.top() + self.bottom())
    }
    pub fn top(&self) -> f32 { self.top.max(0.0) }
    pub fn right(&self) -> f32 { self.right.max(0.0) }
    pub fn bottom(&self) -> f32 { self.bottom.max(0.0) }
    pub fn left(&self) -> f32 { self.left.max(0.0) }
    pub fn all(&self) -> (f32, f32, f32, f32) {
        (self.top(), self.right(), self.bottom(), self.left())
    }
}
