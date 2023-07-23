use crate::{Style, Widget, Pane, Children, NodeId, Node, RadioButton};

pub trait Class<W> {
    fn apply(self, widget: &mut W, style: &mut Style);
}

pub trait StyleClass {
    fn apply(self, style: &mut Style);
}

impl<F, W> Class<W> for F
where
    F: FnOnce(&mut W, &mut Style),
{
    fn apply(self, widget: &mut W, style: &mut Style) {
        self(widget, style);
    }
}

impl<W> Class<W> for () {
    fn apply(self, _widget: &mut W, _style: &mut Style) {}
}

impl<W, C1> Class<W> for (C1,)
where
    C1: Class<W>,
    W: Widget
{
    fn apply(self, widget: &mut W, style: &mut Style) {
        self.0.apply(widget, style);
    }
}

impl<W, C1, C2> Class<W> for (C1, C2)
where
    C1: Class<W>,
    C2: Class<W>,
    W: Widget
{
    fn apply(self, widget: &mut W, style: &mut Style) {
        self.0.apply(widget, style);
        self.1.apply(widget, style);
    }
}

impl<W, C1, C2, C3> Class<W> for (C1, C2, C3)
where
    C1: Class<W>,
    C2: Class<W>,
    C3: Class<W>,
    W: Widget
{
    fn apply(self, widget: &mut W, style: &mut Style) {
        self.0.apply(widget, style);
        self.1.apply(widget, style);
        self.2.apply(widget, style);
    }
}

impl<W, C1, C2, C3, C4> Class<W> for (C1, C2, C3, C4)
where
    C1: Class<W>,
    C2: Class<W>,
    C3: Class<W>,
    C4: Class<W>,
    W: Widget
{
    fn apply(self, widget: &mut W, style: &mut Style) {
        self.0.apply(widget, style);
        self.1.apply(widget, style);
        self.2.apply(widget, style);
        self.3.apply(widget, style);
    }
}

impl<F> StyleClass for F
where
    F: FnOnce(&mut Style),
{
    fn apply(self, style: &mut Style) {
        self(style);
    }
}

impl StyleClass for () {
    fn apply(self, _style: &mut Style) {}
}

impl<C1> StyleClass for (C1,)
where
    C1: StyleClass
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
    }
}

impl<C1, C2> StyleClass for (C1,C2)
where
    C1: StyleClass,
    C2: StyleClass
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
    }
}

impl<C1, C2, C3> StyleClass for (C1,C2,C3)
where
    C1: StyleClass,
    C2: StyleClass,
    C3: StyleClass
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
    }
}

impl<C1, C2, C3, C4> StyleClass for (C1,C2,C3,C4)
where
    C1: StyleClass,
    C2: StyleClass,
    C3: StyleClass,
    C4: StyleClass
{
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
        self.3.apply(style);
    }
}

pub fn pane(
    class: impl StyleClass,
    children: &mut Children,
    children_fn: impl FnOnce(&mut Children)
) -> NodeId {
    
    // Builds node
    let mut node = Node::from_widget(Pane);
    class.apply(&mut node.style);

    // Inserts node and inserts children
    let mut children = children.insert(node);
    children_fn(&mut children);
    children.node_id()
}

pub fn rect(
    class: impl StyleClass,
    children: &mut Children
) -> NodeId {
    let mut node = Node::from_widget(Pane);
    class.apply(&mut node.style);
    let children = children.insert(node);
    children.node_id()
}

pub fn radio_button(class: impl StyleClass, children: &mut Children) -> NodeId {
    let mut node = Node::from_widget(RadioButton::default());
    class.apply(&mut node.style);
    children.insert(node).node_id()
}