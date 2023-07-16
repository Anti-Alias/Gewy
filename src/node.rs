use slotmap::new_key_type;
use crate::{Style, Widget, Pane, Raw};

new_key_type! {
    /// ID of a [`Node`]
    pub struct NodeId;
}

#[derive(Debug)]
pub struct Node {
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) tag: Tag,
    pub(crate) style: Style,
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
    pub fn new(widget: impl Widget, tag: Tag, style: Style) -> Self {
        Self {
            style,
            widget: Box::new(widget),
            tag,
            ..Default::default()
        }
    }
    pub fn from_widget(widget: impl Widget, tag: Tag) -> Self {
        let mut style = Style::default();
        widget.style(&mut style);
        Self {
            style,
            widget: Box::new(widget),
            tag,
            ..Default::default()
        }
    }
    pub fn style(&self) -> &Style { &self.style }
    pub fn style_mut(&mut self) -> &Style { &mut self.style }
    pub fn widget(&self) -> &dyn Widget { self.widget.as_ref() }
    pub fn widget_mut(&mut self) -> &mut dyn Widget { self.widget.as_mut() }
    pub fn tag(&self) -> &Tag { &self.tag }
    pub fn tag_mut(&mut self) -> &mut Tag { &mut self.tag }
    pub fn children(&self) -> &[NodeId] { &self.children_ids }
    pub fn parent(&self) -> Option<NodeId> { self.parent_id }
}


/// Tag that can be attached [`Node`].
/// Identifies which node it is within a [`Widget`].
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Tag {
    /// Empty tag. Events fired from this node will not propogate.
    #[default]
    None,
    /// Represents the "name" of the node, as an integer.
    Name(u16),
    /// Represents the index this node is in within a group.
    /// Useful for identifying nodes within a variable-sized list of nodes, all belonging to the same group (ancestor node).
    Index { group: u16, index: u16 }
}

impl Tag {
    pub const fn name(name: u16) -> Self {
        Self::Name(name)
    }
    pub const fn index(group: u16, index: u16) -> Self {
        Self::Index { group, index }
    }
}