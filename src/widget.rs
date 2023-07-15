use std::fmt::Debug;
use std::any::Any;
use glam::Vec2;
use crate::{NodeId, Gui, Node, Result, Painter, Style, Corners};


/// Represents the "type", state and rendering code of a [`crate::Node`].
pub trait Widget: Debug + Any + 'static {
    /// "Renders" self by spawning its children.
    #[allow(unused_variables)]
    fn render_children<'n>(&mut self, node_id: NodeId, children: NodeChildren<'n>) -> Result<()> {
        Ok(())
    }
    /// Renders self by painting
    fn render_self(&self, _style: &Style, _canvas_size: Vec2, _painter: &mut Painter) {}
}

/// A widget that paints a colored rectangle with rounded corners.
#[derive(Debug)]
pub struct Pane;
impl Widget for Pane {
    fn render_self(&self, style: &Style, canvas_size: Vec2, painter: &mut Painter) {
        let old_color = painter.set_color(style.color);
        let Corners { top_left, top_right, bottom_right, bottom_left } = style.corners;
        painter.rounded_rect(Vec2::ZERO, canvas_size, top_left, top_right, bottom_right, bottom_left);
        painter.color = old_color;
    }
}


/// Allows for only the insertion new nodes as children of a single parent.
pub struct NodeChildren<'n> {
    pub(crate) gui: &'n mut Gui
}

impl<'n> NodeChildren<'n> {
    /// Inserts a new node as the child of another.
    pub fn insert(&mut self, node: Node, parent_id: NodeId) -> Result<NodeId> {
        self.gui.insert(node, parent_id)
    }
}