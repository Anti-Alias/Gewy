use std::{fmt::Debug, any::Any};

use crate::{NodeId, Gui, Node, Result};

/// Represents the "type", state and rendering code of a [`crate::Node`].
pub trait Widget: Debug + Any + 'static {
    /// "Renders" self by spawning its children.
    #[allow(unused_variables)]
    fn render_children<'n>(&mut self, node_id: NodeId, children: NodeChildren<'n>) -> Result<()> {
        Ok(())
    }

    /// Renders self by painting. TODO.
    fn render(&mut self) {}
}

/// Zero-sized sentinal type that is treated specially.
/// Performs no rendering and only serves as a raw container.
#[derive(Debug)]
pub struct Container;
impl Widget for Container {}


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