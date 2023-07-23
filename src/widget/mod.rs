mod button;
pub use button::*;

use std::fmt::Debug;
use std::any::Any;
use glam::Vec2;
use crate::{NodeId, Gui, Node, Result, Painter, Style, RawCorners, GuiError, EventControl, Name, util};


/// Represents the type, state and rendering code of a [`crate::Node`].
pub trait Widget: Any + 'static {

    /// Applies the widget's default style configuration.
    fn style(&self, _style: &mut Style) {}

    /// Spawns descendants, if any at all.
    /// Invoked after node insertion.
    fn descendants(&self, _subtree: Subtree) -> Result<()> {
        Ok(())
    }

    /// Handles an event, and possibly fires a new one.
    fn event(&mut self, _style: &mut Style, _subtree: Subtree, _ctl: &mut EventControl) -> Result<()> {
        Ok(())
    }

    /// Paints self.
    /// Invoked at rendering time.
    fn paint(&self, _style: &Style, _painter: &mut Painter, _canvas: Canvas) {}
}

/// A widget that paints a colored rectangle with rounded corners.
#[derive(Debug)]
pub struct Pane;
impl Widget for Pane {
    fn paint(&self, style: &Style, painter: &mut Painter, canvas: Canvas) {
        util::paint_pane(style, painter, canvas);
    }
}


/// Represents the subtree of a [`Node`].
pub struct Subtree<'n> {
    pub(crate) node_id: NodeId,
    pub(crate) gui: &'n mut Gui
}

impl<'n> Subtree<'n> {

    pub(crate) fn new(node_id: NodeId, gui: &'n mut Gui) -> Self {
        Self { node_id, gui }
    }

    /// ID of the widget's node to build a subtree under.
    pub fn widget_id(&self) -> NodeId {
        self.node_id
    }

    pub fn insert(&mut self, parent_id: NodeId, mut node: Node) -> Result<NodeId> {
        node.ancestor_id = Some(self.node_id);
        self.gui.insert(parent_id, node)
    }

    pub fn get(&mut self, node_id: NodeId) -> Result<&mut Node> {
        if node_id == self.node_id {
            return Err(GuiError::NodeNotFound);
        }
        let node = self.gui.get_mut(node_id)?;
        Ok(node)
    }

    pub fn get_named(&mut self, name: Name) -> Result<&mut Node> {
        self.iter_named(name).next().ok_or(GuiError::NodeNotFound)
    }
    
    pub fn remove(&mut self, node_id: NodeId) -> Option<Node> {
        if node_id == self.node_id {
            return None;
        }
        self.gui.remove(node_id)
    }

    pub fn iter_named(&mut self, name: Name) -> impl Iterator<Item = &mut Node> + '_ {
        self.gui
            .iter_named(name)
            .filter(|node| node.ancestor_id == Some(self.node_id))
    }
}

/// Area for [`Painter`] to draw in.
/// The boundary of the painter ranges from the top left (0.0, 0.0) to the bottom right (size.x, size.y).
/// Widgets must not paint outside of this boundary.
/// Stores "raw" values from the style of the node being drawn.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Canvas {
    /// Size of the canvas in pixels.
    /// Widgets must not paint outside of the range [0.0, 0.0] - [size.x, size.y].
    pub size: Vec2,
    /// Raw radiuses of corners in pixels.
    pub corners: RawCorners
}

impl Canvas {
    pub fn center(&self) -> Vec2 {
        self.size / 2.0
    }
}