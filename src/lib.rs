mod node;
mod gewy;
mod style;
mod widget;
mod event;
mod color;
mod paint;
mod err;
mod math;
mod raw;
mod extensions;
mod input;

// Extended APIs
pub mod util;
pub mod dsl;

// Backends/integrations
pub mod winit;
pub mod wgpu;

// Re-exports internals
pub use node::*;
pub use gewy::*;
pub use style::*;
pub use widget::*;
pub use event::*;
pub use color::*;
pub use paint::*;
pub use math::*;
pub use err::*;
pub use raw::*;
pub use input::*;

// Re-exports external dependencies
pub use glam::*;