use std::fmt::Debug;
use std::any::Any;
use std::f32::consts::{FRAC_PI_2, PI};
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

        // "Pushes" painter state.
        let color = painter.color;
        painter.color = style.color;

        // Gets radiuses and centers of the corner circles.
        let half_extent = (canvas_size / 2.0).min_element();
        let Corners { top_left, top_right, bottom_right, bottom_left } = style.corners;
        let top_left = top_left.min(half_extent);
        let top_right = top_right.min(half_extent);
        let bottom_right = bottom_right.min(half_extent);
        let bottom_left = bottom_left.min(half_extent);
        let c_br = Vec2::new(canvas_size.x - bottom_right, canvas_size.y - bottom_right);
        let c_bl = Vec2::new(bottom_left, canvas_size.y - bottom_left);
        let c_tl = Vec2::new(top_left, top_left);
        let c_tr = Vec2::new(canvas_size.x - top_right, top_right);
        
        // Paints with shape painter.
        painter.shape()
            .quarter_circle(c_br, bottom_right, 0.0)
            .quarter_circle(c_bl, bottom_left, FRAC_PI_2)
            .quarter_circle(c_tl, top_left, PI)
            .quarter_circle(c_tr, top_right, 3.0 * FRAC_PI_2);

        // "Pops" painter state.
        painter.color = color;
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