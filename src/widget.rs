use std::{fmt::Debug, any::Any, f32::consts::{FRAC_PI_2, PI, TAU, FRAC_PI_3}};

use glam::Vec2;

use crate::{NodeId, Gui, Node, Result, Painter, Style, Corners};

const EPS: f32 = 0.001;

/// Represents the "type", state and rendering code of a [`crate::Node`].
pub trait Widget: Debug + Any + 'static {
    /// "Renders" self by spawning its children.
    #[allow(unused_variables)]
    fn render_children<'n>(&mut self, node_id: NodeId, children: NodeChildren<'n>) -> Result<()> {
        Ok(())
    }

    /// Renders self by painting. TODO.
    fn render_self(&self, style: &Style, canvas_size: Vec2, painter: &mut Painter) {}
}

/// Zero-sized sentinal type that is treated specially.
/// Performs no rendering and only serves as a raw container.
#[derive(Debug)]
pub struct Container;
impl Widget for Container {

    fn render_self(&self, style: &Style, canvas_size: Vec2, p: &mut Painter) {

        // "Push" painter state.
        let color = p.color;
        p.color = style.color;

        // Get radiuses and centers of the corner circles.
        let Corners { top_left, top_right, bottom_right, bottom_left } = style.corners;
        let c_br = Vec2::new(canvas_size.x - bottom_right, canvas_size.y - bottom_right);
        let c_bl = Vec2::new(bottom_left, canvas_size.y - bottom_left);
        let c_tl = Vec2::new(top_left, top_left);
        let c_tr = Vec2::new(canvas_size.x - top_right, top_right);
        
        // Paints with shape painter.
        let mut sp = p.shape();
        sp.quarter_circle(c_br, bottom_right, 0.0);
        sp.quarter_circle(c_bl, bottom_left, FRAC_PI_2);
        sp.quarter_circle(c_tl, top_left, PI);
        sp.quarter_circle(c_tr, top_right, 3.0 * FRAC_PI_2);
        drop(sp);

        // "Pop" painter state.
        p.color = color;
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