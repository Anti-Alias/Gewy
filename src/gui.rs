use glam::Vec2;
use slotmap::SlotMap;
use crate::{GUiError, Result, NodeChildren, Node, NodeId, NodePath, NodePathElem, Rect, Layout, JustifyContent, extensions::VecExtensions};

/// Represents a graphical user interface, and a torage of [`Node`]s.
#[derive(Debug, Default)]
pub struct Gui {
    storage: SlotMap<NodeId, Node>,
    root_id: NodeId
}

impl Gui {

    /// Creates a new [`Gui`] instance alongside the id of the root node.
    pub fn new(root: Node, size: Vec2) -> (NodeId, Self) {
        let mut storage = SlotMap::<NodeId, Node>::default();
        let root_node_id = storage.insert(root);
        let mut slf = Self { storage, root_id: root_node_id };
        slf.resize(size);
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
        parent.children_ids.push(node_id);

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
            let child_idx = parent.children_ids.iter()
                .position(|child_id| child_id == &node_id)
                .unwrap();
            parent.children_ids.remove(child_idx);
        }
        for child_id in std::mem::take(&mut node.children_ids) {
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

    pub unsafe fn get_mut_unsafe(&mut self, node_id: NodeId) -> Result<&'static mut Node> {
        std::mem::transmute(self.get_mut(node_id))
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

    pub fn resize(&mut self, size: Vec2) {
        let node = self.get_mut(self.root_id).unwrap();
        node.cache.region = Rect::new(Vec2::ZERO, size);
        self.compute_region(self.root_id);
    }

    fn compute_region<'a>(&'a mut self, node_id: NodeId) {
        
        // Gets children of node
        let node = self.get(node_id).unwrap();
        let children: &[NodeId] = unsafe { std::mem::transmute(node.children()) };
        if children.is_empty() { return }

        // Computes regions of children
        let layout = node.style.layout;
        let region = node.cache.region;
        self.compute_regions(children, region, layout, layout.direction.is_row());
    }

    fn compute_regions(
        &mut self,
        node_ids: &[NodeId],
        parent_region: Rect,
        parent_layout: Layout,
        is_row: bool
    ) {
        // Computes basis of each node, and returns the total
        let parent_size = parent_region.size.flip(!is_row);
        let mut basis_total = 0.0;
        for id in node_ids {
            let node = self.get_mut(*id).unwrap();
            let basis = node.style.get_basis(parent_size.x, is_row);
            node.cache.basis = basis;
            basis_total += basis;
        }

        // Growing scenario
        if basis_total <= parent_size.x {
            self.grow_children(node_ids, basis_total, parent_size, parent_layout);
            for id in node_ids {
                let node = self.get_mut(*id).unwrap();
                node.cache.region = node.cache.region.flip(!is_row);
                node.cache.region.pos += parent_region.pos;              
                self.compute_region(*id);
            }
        }

        // Shrinking scenario
        else {
            unimplemented!("Shrinking not yet supported");
        }
    }

    fn grow_children(
        &mut self,
        node_ids: &[NodeId],
        basis_total: f32,
        parent_size: Vec2,
        parent_layout: Layout
    ) {

        // Computes the total growth of all the nodes
        let mut grow_total = 0.0;
        for id in node_ids {
            let node = self.get_mut(*id).unwrap();
            grow_total += node.style.config.grow.max(0.0);
        }
        grow_total = grow_total.max(1.0);

        // Sizes nodes and lines them of from left to right with no spacing.
        let remaining_width = parent_size.x - basis_total;
        let mut total_width = 0.0;
        for id in node_ids {
            let node = self.get_mut(*id).unwrap();
            let grow_perc = node.style.config.grow / grow_total;
            let node_width = node.cache.basis + remaining_width * grow_perc;
            let node_height = parent_size.y;
            node.cache.region = Rect::new(
                Vec2::new(total_width, 0.0),
                Vec2::new(node_width, node_height)
            );
            total_width += node_width;
        }

        // Determines offset and spacing of nodes
        let (offset, spacing) = match parent_layout.justify_content {
            JustifyContent::Start => return,
            JustifyContent::End => (parent_size.x - total_width, 0.0),
            JustifyContent::Center => (parent_size.x/2.0 - total_width/2.0, 0.0),
            _ => todo!()
        };

        // Applies offset and spacing to nodes
        for (i, id) in node_ids.iter().enumerate() {
            let node = self.get_mut(*id).unwrap();
            node.cache.region.pos += offset;
            node.cache.region.pos += (i as f32) * spacing;
        }
    }
}


#[cfg(test)]
mod test {
    use glam::Vec2;

    use crate::{Gui, Node};

    #[test]
    fn test_insert() {
        let (root_id, mut nodes) = Gui::new(Node::default(), Vec2::new(100.0, 100.0));
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
        let (root_id, mut nodes) = Gui::new(Node::default(), Vec2::new(100.0, 100.0));
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