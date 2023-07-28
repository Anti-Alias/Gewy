use glam::Vec2;

use crate::Rect;

pub type RawMargin = RawSides;
pub type RawPadding = RawSides;

/// Stores raw values computed during the layout phase.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub(crate) struct Raw {
    pub region: Rect,
    pub margin: RawMargin,
    pub padding: RawPadding,
    pub corners: RawCorners
}

impl Raw {

    // Region of the node containing the content + padding.
    pub fn padding_region(&self) -> Rect {
        let margin = &self.margin;
        let top_left = Vec2::new(margin.left, margin.top);
        let bottom_right = Vec2::new(margin.right, margin.bottom);
        let position = self.region.position + top_left;
        let size = self.region.size - top_left - bottom_right;
        Rect { position, size }.non_negative()
    }

    // Region of the node containing only the content.
    pub fn content_region(&self) -> Rect {
        let (margin, padding) = (&self.margin, &self.padding);
        let top_left = Vec2::new(margin.left, margin.top) + Vec2::new(padding.left, padding.top);
        let bottom_right = Vec2::new(margin.right, margin.bottom) + Vec2::new(padding.right, padding.bottom);
        let position = self.region.position + top_left;
        let size = self.region.size - top_left - bottom_right;
        Rect { position, size }.non_negative()
    }

    // Width of the content region.
    pub fn content_width(&self) -> f32 {
        self.region.size.x - (self.margin.left + self.margin.right + self.padding.left + self.padding.right)
    }

    // Sets the content width.
    pub fn set_content_width(&mut self, content_width: f32) {
        self.region.size.x = content_width + (self.margin.left + self.margin.right + self.padding.left + self.padding.right);
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
impl RawCorners {
    pub(crate) fn round(self, unit: f32) -> Self {
        Self {
            top_left: (self.top_left / unit).round() * unit,
            top_right: (self.top_right / unit).round() * unit,
            bottom_right: (self.bottom_right / unit).round() * unit,
            bottom_left: (self.bottom_left / unit).round() * unit
        }
    }
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
