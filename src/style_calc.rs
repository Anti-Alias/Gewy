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

pub fn basis_px<A: Axis>(style: &Style, parent_width: f32) -> f32 {
    match (style.config.basis, style.get_width::<A>()) {
        (Val::Px(px), _) => px,
        (Val::Pc(pc), _) => pc * parent_width,
        (Val::Auto, Val::Px(px)) => px,
        (Val::Auto, Val::Pc(pc)) => pc * parent_width,
        (Val::Auto, Val::Auto) => parent_width
    }
}
