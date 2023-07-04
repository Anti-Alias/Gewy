use slotmap::SlotMap;
use crate::{GUiError, Result, NodeChildren, Node, NodeId, NodePath, NodePathElem};

/// Represents a graphical user interface, and a torage of [`Node`]s.
#[derive(Debug, Default)]
pub struct Gui {
    storage: SlotMap<NodeId, Node>,
    root_id: NodeId
}

impl Gui {

    /// Creates a new [`Gui`] instance alongside the id of the root node.
    pub fn new(root: Node) -> (NodeId, Self) {
        let mut storage = SlotMap::<NodeId, Node>::default();
        let root_node_id = storage.insert(root);
        let slf = Self { storage, root_id: root_node_id };
        (root_node_id, slf)
    }

    /// Id of the root node.
    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    /// Inserts as a child of another.
    /// Returns id of node inserted.
    pub fn insert(&mut self, mut node: Node, parent_id: NodeId) -> Result<NodeId> {

        // Stores node as a child of its parent
        node.parent_id = Some(parent_id);
        let node_id = self.storage.insert(node);
        let Some(parent) = self.storage.get_mut(parent_id) else {
            self.storage.remove(node_id);
            return Err(GUiError::ParentNodeNotFound.into())
        };
        parent.children.push(node_id);

        // Configures widget
        unsafe {
            let gui = self as *mut Self;
            let gui = &mut *gui;
            let children = NodeChildren { gui };
            let node = self.storage.get_mut(node_id).unwrap();
            node.widget.render_children(node_id, children)?;    // Using node past here is unsafe!!!
        };

        // Done
        Ok(node_id)
    }

    /// Removes a node and all of its children.
    /// Returns the node removed if it exists.
    /// Returns None if the node was not found or was the root.
    pub fn remove(&mut self, node_id: NodeId) -> Option<Node> {
        if node_id == self.root_id {
            return None;
        }
        let Some(mut node) = self.storage.remove(node_id) else {
            return None;
        };
        if let Some(parent_id) = node.parent_id {
            let parent = self.storage.get_mut(parent_id).unwrap();
            let child_idx = parent.children.iter()
                .position(|child_id| child_id == &node_id)
                .unwrap();
            parent.children.remove(child_idx);
        }
        for child_id in std::mem::take(&mut node.children) {
            self.remove(child_id);
        };
        Some(node)
    }

    pub fn get(&self, node_id: NodeId) -> Result<&Node> {
        self.storage.get(node_id).ok_or(GUiError::NodeNotFound)
    }
    
    pub fn get_mut(&mut self, node_id: NodeId) -> Result<&mut Node> {
        self.storage.get_mut(node_id).ok_or(GUiError::NodeNotFound)
    }

    pub fn path_to(&self, node_id: NodeId) -> Result<NodePath> {
        let mut path = self.reverse_path_to(node_id)?;
        path.0.reverse();
        Ok(path)
    }

    pub fn reverse_path_to(&self, node_id: NodeId) -> Result<NodePath> {
        let mut result = Vec::new();
        let mut node = self.get(node_id)?;
        result.push(NodePathElem { id: node_id, tag: node.tag, widget_type: node.widget.type_id() });
        while let Some(node_id) = node.parent_id {
            node = self.get(node_id).unwrap();
            result.push(NodePathElem { id: node_id, tag: node.tag, widget_type: node.widget.type_id() });
        }
        Ok(NodePath(result))
    }

    // Gets all of the nodes for all of the ids supplied.
    pub(crate) fn get_all<const N: usize>(&mut self, ids: [NodeId; N]) -> [&mut Node; N] {
        self.storage.get_disjoint_mut(ids).unwrap()
    }
}


#[cfg(test)]
mod test {
    use crate::{Gui, Node};

    #[test]
    fn test_insert() {
        let (root_id, mut nodes) = Gui::new(Node::default());
        let child_1_id = nodes.insert(Node::default(), root_id).unwrap();
        let child_2_id = nodes.insert(Node::default(), root_id).unwrap();
        
        let root = nodes.get(root_id).unwrap();
        assert_eq!(2, root.children().len());

        let child_1 = nodes.get(child_1_id).unwrap();
        assert_eq!(0, child_1.children().len());
        assert_eq!(Some(root_id), child_1.parent());

        let child_2 = nodes.get(child_2_id).unwrap();
        assert_eq!(0, child_2.children().len());
        assert_eq!(Some(root_id), child_2.parent());
    }

    #[test]
    fn test_remove() {
        let (root_id, mut nodes) = Gui::new(Node::default());
        let child_1_id = nodes.insert(Node::default(), root_id).unwrap();
        let child_2_id = nodes.insert(Node::default(), root_id).unwrap();
        
        let root = nodes.get(root_id).unwrap();
        assert_eq!(2, root.children().len());

        nodes.remove(child_1_id).unwrap();
        let root = nodes.get(root_id).unwrap();
        assert_eq!(1, root.children().len());

        nodes.remove(child_2_id).unwrap();
        let root = nodes.get(root_id).unwrap();
        assert_eq!(0, root.children().len());
        
        assert!(nodes.remove(root_id).is_none());
    }
}