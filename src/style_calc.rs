use crate::{Style, Val, Axis};

pub fn raw_width_px(style: &Style) -> f32 {
    match style.width {
        crate::Val::Px(px) => px,
        _ => 0.0
    }
}

pub fn raw_height_px(style: &Style) -> f32 {
    match style.height {
        crate::Val::Px(px) => px,
        _ => 0.0
    }
}
