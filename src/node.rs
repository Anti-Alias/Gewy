use slotmap::new_key_type;
use crate::{Style, Widget, Pane, Raw};

new_key_type! {
    /// ID of a [`Node`]
    pub struct NodeId;
}

#[derive(Debug)]
pub struct Node {
    pub(crate) style: Style,
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) tag: Tag,
    pub(crate) children_ids: Vec<NodeId>,
    pub(crate) parent_id: Option<NodeId>,
    pub(crate) raw: Raw
}

impl Default for Node {
    fn default() -> Self {
        Self {
            style: Default::default(),
            widget: Box::new(Pane),
            tag: Default::default(),
            children_ids: Vec::new(),
            parent_id: None,
            raw: Default::default()
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
    pub fn children(&self) -> &[NodeId] { &self.children_ids }
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