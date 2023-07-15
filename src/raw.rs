use crate::{Rect, Canvas};

/// Stores raw values computed during the layout phase.
/// Units of all values are stored in pixels.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub(crate) struct Raw {
    pub region: Rect,
    pub basis: f32,
    pub corners: RawCorners
}

impl Raw {
    pub fn canvas(&self) -> Canvas {
        Canvas {
            size: self.region.size,
            corners: self.corners
        }
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