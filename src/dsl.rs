use crate::{Style, Pane, Descendants, NodeId, Node, RadioButton};

pub trait Class {
    fn apply(self, style: &mut Style);
}

impl<F> Class for F
where
    F: FnOnce(&mut Style),
{
    fn apply(self, style: &mut Style) {
        self(style);
    }
}

impl Class for () {
    fn apply(self, _style: &mut Style) {}
}

impl<C1> Class for (C1,)
where
    C1: Class
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
    }
}

impl<C1, C2> Class for (C1,C2)
where
    C1: Class,
    C2: Class
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
    }
}

impl<C1, C2, C3> Class for (C1,C2,C3)
where
    C1: Class,
    C2: Class,
    C3: Class
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
    }
}

impl<C1, C2, C3, C4> Class for (C1,C2,C3,C4)
where
    C1: Class,
    C2: Class,
    C3: Class,
    C4: Class
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
        self.3.apply(style);
    }
}

pub fn pane(class: impl Class, descendants: &mut Descendants, descendants_fn: impl FnOnce(&mut Descendants)) -> NodeId {
    let mut node = Node::from_widget(Pane);
    class.apply(&mut node.style);
    let mut descendants = descendants.insert(node);
    descendants_fn(&mut descendants);
    descendants.node_id()
}

pub fn rect(class: impl Class, descendants: &mut Descendants) -> NodeId {
    let mut node = Node::from_widget(Pane);
    class.apply(&mut node.style);
    let descendants = descendants.insert(node);
    descendants.node_id()
}

pub fn radio_button(class: impl Class, descendants: &mut Descendants) -> NodeId {
    let mut node = Node::from_widget(RadioButton::default());
    class.apply(&mut node.style);
    descendants.insert(node).node_id()
}