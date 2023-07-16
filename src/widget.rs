use std::fmt::Debug;
use std::any::Any;
use glam::Vec2;
use crate::{NodeId, Gui, Node, Result, Painter, Style, RawCorners};


/// Represents the type, state and rendering code of a [`crate::Node`].
pub trait Widget: Debug + Any + 'static {
    /// Applies the widget's default style configuration.
    fn style<'n>(&self, _style: &mut Style) {}
    /// Spawns children of itself.
    /// Invoked after node insertion.
    fn children<'n>(&self, _children: NodeChildren<'n>) -> Result<()> { Ok(()) }
    /// Paints self.
    /// Invoked at rendering time.
    fn paint(&self, _style: &Style, _canvas: Canvas, _painter: &mut Painter) {}
}

/// A widget that paints a colored rectangle with rounded corners.
#[derive(Debug)]
pub struct Pane;
impl Widget for Pane {
    fn paint(&self, style: &Style, canvas: Canvas, painter: &mut Painter) {
        let old_color = painter.set_color(style.color);
        let Canvas { size, corners } = canvas;
        painter.rounded_rect(Vec2::ZERO, size, corners.top_left, corners.top_right, corners.bottom_right, corners.bottom_left);
        painter.color = old_color;
    }
}


/// Allows for only the insertion new nodes as children of a single parent.
pub struct NodeChildren<'n> {
    pub(crate) gui: &'n mut Gui,
    pub(crate) parent_id: NodeId
}

impl<'n> NodeChildren<'n> {
    /// Inserts a new node as the child of another.
    pub fn insert(&mut self, node: Node) -> Result<NodeId> {
        self.gui.insert(node, self.parent_id)
    }
}

/// Area for [`Painter`] to draw in.
/// The boundary of the painter ranges from the top left (0.0, 0.0) to the bottom right (size.x, size.y).
/// Widgets must not paint outside of this boundary.
/// Stores "raw" values from the style of the node being drawn.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Canvas {
    /// Size of the canvas in pixels.
    pub size: Vec2,
    /// Raw radiuses of corners in pixels.
    pub corners: RawCorners
}