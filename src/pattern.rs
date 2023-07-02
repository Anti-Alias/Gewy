use std::any::TypeId;

use crate::{NodeId, Tag, Widget, NodePath, NodePathElem};

/// Represents a pattern of nodes to be matched against a [`NodePath`].
#[derive(Eq, PartialEq, Debug)]
pub struct NodePattern(Vec<NodePatternElem>);

impl NodePattern {

    /// Creates a new [`NodePattern`] with a root node type.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Push a descendant to the pattern. Widget is not allowed.
    pub fn push(&mut self, descendant: impl Into<NodePatternElem>) -> &mut Self {
        let descendant = descendant.into();
        self.0.push(descendant);
        self
    }   

    // Checks if the pattern matches the path.
    // Returns true if it was a full match.
    // May partially write to vec even if a failed match.
    // If match is successful, nodes will be parallel to pattern.
    pub(crate) fn matches(
        &self,
        path: &NodePath,
        nodes: &mut Vec<NodeId>
    ) -> bool {

        // Sanity checks
        let path = &path.0;
        let pattern = &self.0;
        if pattern.is_empty() { return true; }
        if pattern.len() > path.len() { return false; }

        // Ensures tails match
        let mut path_idx = path.len() - 1;
        let mut patt_idx = pattern.len() - 1;
        let path_tail = &path[path_idx];
        let patt_tail = &pattern[patt_idx];
        if patt_tail.matches(path_tail) {
            nodes.clear();
            nodes.push(path_tail.id);
            if patt_idx == 0 { return true }
        }
        else {
            return false;
        }
        path_idx -= 1;
        patt_idx -= 1;
        
        // Iterates from tail to head of both.
        loop {
            let patt_elem = &pattern[patt_idx];
            loop {
                let path_elem = &path[path_idx];
                if patt_elem.matches(&path_elem) {
                    nodes.push(path_elem.id);
                    break;
                }
                if path_idx == 0 { return false }
                path_idx -= 1;
            }
            if patt_idx == 0 {
                nodes.reverse();
                return true;
            }
            patt_idx -= 1;
        }
    }
}

/// Companion to [`crate::Tag`] used for matching a [`NodePath`] against a [`NodePattern`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NodePatternElem {
    WidgetType(TypeId),
    Name(&'static str),
    NameStartsWith(&'static str),
    NameEndsWith(&'static str), 
    NameAny,
    Index(usize),
    IndexAny
}

impl NodePatternElem {
    fn matches(self, node: &NodePathElem) -> bool {
        match self {
            Self::WidgetType(matcher_type) => matcher_type == node.widget_type,
            Self::Name(matcher_name) => {
                let Tag::Name(node_name) = node.tag else { return false };
                matcher_name == node_name
            },
            Self::NameStartsWith(matcher_name) => {
                let Tag::Name(node_name) = node.tag else { return false };
                node_name.starts_with(matcher_name)
            },
            Self::NameEndsWith(matcher_name) => {
                let Tag::Name(node_name) = node.tag else { return false };
                node_name.ends_with(matcher_name)
            },
            Self::NameAny => {
                let Tag::Name(_) = node.tag else { return true };
                false
            },
            Self::Index(matcher_index) => {
                let Tag::Index(node_index) = node.tag else { return false };
                matcher_index == node_index
            }
            Self::IndexAny => {
                if let Tag::Index(_) = node.tag { return true };
                false
            }
        }
    }
}

impl From<&'static str> for NodePatternElem {
    fn from(value: &'static str) -> Self {
        NodePatternElem::Name(value)
    }
}

impl From<usize> for NodePatternElem {
    fn from(value: usize) -> Self {
        NodePatternElem::Index(value)
    }
}

pub fn widget<W: Widget>() -> NodePatternElem {
    let w_type = TypeId::of::<W>();
    NodePatternElem::WidgetType(w_type)
}

pub const fn starts(name: &'static str) -> NodePatternElem {
    NodePatternElem::NameStartsWith(name)
}

pub const fn ends(name: &'static str) -> NodePatternElem {
    NodePatternElem::NameEndsWith(name)
}

pub const fn any_name() -> NodePatternElem {
    NodePatternElem::NameAny
}

pub const fn any_index() -> NodePatternElem {
    NodePatternElem::IndexAny
}

/// Macro that generates a [`NodePattern`]
macro_rules! node_pattern {
    [$widget_type:ident] => {{
        let mut pattern = NodePattern::new();
        let w_type = std::any::TypeId::of::<$widget_type>();
        pattern.push(NodePatternElem::WidgetType(w_type));
        pattern
    }};
    ($($elem:expr),*) => {{
        let mut pattern = NodePattern::new();
        $(pattern.push($elem);)*
        pattern
    }};
    ($widget_type:ty; $($elem:expr),*) => {{
        let mut pattern = NodePattern::new();
        let w_type = std::any::TypeId::of::<$widget_type>();
        pattern.push(NodePatternElem::WidgetType(w_type));
        $(pattern.push($elem);)*
        pattern
    }}
}


#[cfg(test)]
mod test {
    use crate::*;
    
    #[derive(Debug)]
    struct Root;
    impl Widget for Root {}

    #[derive(Debug)]
    struct Child;
    impl Widget for Child {}

    #[test]
    fn pattern_matching() {

        //////////////////////////////////////////////////
        //            root:Root
        //         /             \
        //     steve:Child    sarah:Child
        //      /              /           \
        //  bob:Child      derp:Child   derpette:Child
        //////////////////////////////////////////////////

        // Makes nodes.
        let root = Node::new(Style::default(), Root);
        let steve = Node::tagged("steve", Style::default(), Child);
        let sarah = Node::tagged("sarah", Style::default(), Child);
        let bob = Node::tagged("bob", Style::default(), Child);
        let derp = Node::tagged(0, Style::default(), Child);
        let derpette = Node::tagged(1, Style::default(), Child);

        // Stores nodes, matching the tree structure above.
        let (root_id, mut storage) = NodeStorage::new(root);
        let steve_id = storage.insert(steve, root_id).unwrap();
        let sarah_id = storage.insert(sarah, root_id).unwrap();
        let bob_id = storage.insert(bob, steve_id).unwrap();
        let derp_id = storage.insert(derp, sarah_id).unwrap();
        let derpette_id = storage.insert(derpette, sarah_id).unwrap();

        // Matches root.
        let path = storage.path_to(root_id).unwrap();
        let pattern = node_pattern!(Root);
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id], nodes);

        // Matches steve.
        let path = storage.path_to(steve_id).unwrap();
        let pattern = node_pattern!(Root; "steve");
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id, steve_id], nodes);

        // Matches sarah.
        let path = storage.path_to(sarah_id).unwrap();
        let pattern = node_pattern!(Root; "sarah");
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id, sarah_id], nodes);

        // Matches bob with an explicit path.
        let path = storage.path_to(bob_id).unwrap();
        let pattern = node_pattern!(Root; "steve", "bob");
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id, steve_id, bob_id], nodes);

        // Matches bob with an implicit path.
        let path = storage.path_to(bob_id).unwrap();
        let pattern = node_pattern!(Root; "bob");
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id, bob_id], nodes);

        // Matches derp
        let path = storage.path_to(derp_id).unwrap();
        let pattern = node_pattern!(Root; "sarah", 0);
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id, sarah_id, derp_id], nodes);

        let mut pattern = NodePattern::new();
        pattern.push(widget::<Root>());
        pattern.push("steve");
        pattern.push(1);

        // Matches derpette
        let path = storage.path_to(derpette_id).unwrap();
        let pattern = node_pattern!(Root; "sarah", 1);
        let mut nodes = Vec::new();
        let matches = pattern.matches(&path, &mut nodes);
        assert!(matches);
        assert_eq!(vec![root_id, sarah_id, derpette_id], nodes);
    }
}