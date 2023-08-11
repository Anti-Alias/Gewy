use slotmap::new_key_type;
use crate::{Style, Widget, Pane, Raw};

/// Name of a [`Node`].
pub type Name = u16;

new_key_type! {
    /// ID of a [`Node`]
    pub struct NodeId;
}

/// Element in a [`crate::Gewy`] tree.
/// Stores a [`Widget`] and a [`Style`], and may or may not have child [`Node`]s.
pub struct Node {
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) name: Option<Name>,
    pub(crate) style: Style,
    pub(crate) children_ids: Vec<NodeId>,
    pub(crate) parent_id: Option<NodeId>,
    pub(crate) ancestor_id: Option<NodeId>,
    pub(crate) raw: Raw
}

impl Default for Node {
    fn default() -> Self {
        Self {
            style: Default::default(),
            widget: Box::new(Pane),
            name: None,
            children_ids: Vec::new(),
            parent_id: None,
            ancestor_id: None,
            raw: Default::default()
        }
    }
}

impl Node {
    pub fn new(widget: impl Widget, style: Style) -> Self {
        Self {
            style,
            widget: Box::new(widget),
            ..Default::default()
        }
    }
    pub fn from_widget(widget: impl Widget) -> Self {
        let mut style = Style::default();
        widget.style(&mut style);
        Self {
            style,
            widget: Box::new(widget),
            ..Default::default()
        }
    }
    pub fn with_name(mut self, name: Option<Name>) -> Self {
        self.name = name;
        self
    }
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn style(&self) -> &Style { &self.style }
    pub fn style_mut(&mut self) -> &Style { &mut self.style }
    pub fn widget(&self) -> &dyn Widget { self.widget.as_ref() }
    pub fn widget_mut(&mut self) -> &mut dyn Widget { self.widget.as_mut() }
    pub fn name(&self) -> Option<Name> { self.name }
    pub fn name_mut(&mut self) -> Option<&mut Name> { self.name.as_mut() }
    pub fn children(&self) -> &[NodeId] { &self.children_ids }
    pub fn parent(&self) -> Option<NodeId> { self.parent_id }
}