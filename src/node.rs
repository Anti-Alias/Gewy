use std::any::TypeId;

use slotmap::new_key_type;
use crate::{Style, Widget, Container, Rect};

new_key_type! {
    /// ID of a [`Node`]
    pub struct NodeId;
}

#[derive(Debug)]
pub struct Node {
    pub(crate) style: Style,
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) tag: Tag,
    pub(crate) children: Vec<NodeId>,
    pub(crate) parent_id: Option<NodeId>,
    pub(crate) computed_region: Rect
}

impl Default for Node {
    fn default() -> Self {
        Self {
            style: Default::default(),
            widget: Box::new(Container),
            tag: Default::default(),
            children: Vec::new(),
            parent_id: None,
            computed_region: Rect::default()
        }
    }
}

impl Node {

    /// Makes a new node.
    pub fn new(style: Style, widget: impl Widget) -> Self {
        Self {
            style,
            widget: Box::new(widget),
            ..Default::default()
        }
    }

    /// Makes a named node.
    pub fn tagged(tag: impl Into<Tag>, style: Style, widget: impl Widget) -> Self {
        Self {
            style,
            widget: Box::new(widget),
            tag: tag.into(),
            ..Default::default()
        }
    }

    /// Makes an indexed node.
    pub fn indexed(index: usize, style: Style, widget: impl Widget) -> Self {
        Self {
            style,
            widget: Box::new(widget),
            tag: Tag::Index(index),
            ..Default::default()
        }
    }

    pub fn style(&self) -> &Style { &self.style }
    pub fn style_mut(&mut self) -> &Style { &mut self.style }
    pub fn get_widget(&self) -> &dyn Widget { self.widget.as_ref() }
    pub fn get_widget_mut(&mut self) -> &mut dyn Widget { self.widget.as_mut() }
    pub fn get_tag(&self) -> &Tag { &self.tag }
    pub fn get_tag_mut(&mut self) -> &mut Tag { &mut self.tag }
    pub fn children(&self) -> &[NodeId] { &self.children }
    pub fn parent(&self) -> Option<NodeId> { self.parent_id }
}


/// Tag that can be attached [`Node`].
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Tag {
    #[default]
    Empty,
    Name(&'static str),
    Index(usize)
}

impl From<()> for Tag {
    fn from(_value: ()) -> Self {
        Self::Empty
    }
}

impl From<&'static str> for Tag {
    fn from(value: &'static str) -> Self {
        Self::Name(value)
    }
}

impl From<usize> for Tag {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}


/// A reference to a [`Node`] alongside it's id.
#[derive(Debug)]
pub struct NodeInfo<'n> {
    pub node: &'n mut Node,
    pub id: NodeId
}

// Path to a single node with the first element being the root node and the last being the node in question.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodePath(pub(crate) Vec<NodePathElem>);

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct NodePathElem {
    pub(crate) id: NodeId,          // ID of the node
    pub(crate) tag: Tag,            // Tag of the node
    pub(crate) widget_type: TypeId  // Widget type of the node
}