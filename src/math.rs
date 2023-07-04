/// Simple rectangle struct.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    /// Left-most corner.
    pub x: f32,
    /// Right-most corner.
    pub y: f32,
    /// Width in pixels.
    pub width: f32,
    /// Height in pixels.
    pub height: f32
}