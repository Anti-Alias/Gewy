use glam::Vec2;
use slotmap::SlotMap;
use crate::{GUiError, Result, NodeChildren, Node, NodeId, NodePath, NodePathElem, Rect, Layout, style_calc::basis_px, JustifyContent, Axis, XAxis, YAxis, extensions::VecExtensions, node};

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
        let node = unsafe { self.get_mut_unsafe(self.root_id).unwrap() };
        node.cache.region = Rect::new(Vec2::ZERO, size);
        self.compute_regions_of(node, &mut Vec::new());
    }

    fn compute_regions_of<'a>(&'a mut self, node: &mut Node, children: &mut Vec<&'a mut Node>) {
        
        // Gets children
        let layout_direction = node.style.layout.direction;
        unsafe { self.get_nodes(children, &node.children_ids); }
        if children.is_empty() { return }

        // Computes their regions
        if layout_direction.is_reverse() { children.reverse() };
        let region = node.cache.region;
        match layout_direction.is_row() {
            true => self.compute_regions::<XAxis>(children, region, node.style.layout),
            false => self.compute_regions::<YAxis>(children, region, node.style.layout)
        }
    }

    fn compute_regions<A: Axis>(
        &mut self,
        nodes: &mut Vec<&mut Node>,
        parent_region: Rect,
        parent_layout: Layout
    ) {
        // Computes basis of each node, and returns the total
        let parent_width = parent_region.get_width::<A>();
        let basis_total: f32 = nodes.iter_mut()
            .map(|n| {
                let basis = n.style.get_basis_px::<A>(parent_width);
                n.cache.basis = basis;
                basis
            })
            .sum();

        // Growing scenario
        if basis_total <= parent_width {
            self.grow_children::<A>(nodes, basis_total, parent_region, parent_layout);
        }

        // Shrinking scenario
        else {
            unimplemented!("Shrinking not yet supported");
        }
    }

    fn grow_children<A: Axis>(
        &mut self,
        nodes: &mut Vec<&mut Node>,
        basis_total: f32,
        parent_region: Rect,
        parent_layout: Layout
    ) {
        let parent_size = parent_region.size;
        let width_remaining = parent_size.get_x::<A>() - basis_total;
        let ps = parent_size.flip::<A>();
        let starting_pos = parent_region.pos + match parent_layout.justify_content {
            JustifyContent::Start => Vec2::ZERO,
            JustifyContent::End => Vec2::new(ps.x - basis_total, 0.0).flip::<A>(),
            JustifyContent::Center => Vec2::new(ps.x/2.0 - basis_total / 2.0, ps.y).flip::<A>(),
            JustifyContent::SpaceBetween => todo!(),
            JustifyContent::SpaceAround => todo!(),
            JustifyContent::SpaceEvenly => todo!(),
        };
        let space_between = match parent_layout.justify_content {
            JustifyContent::Start | JustifyContent::End | JustifyContent::Center => 0.0,
            JustifyContent::SpaceBetween => todo!(),
            JustifyContent::SpaceAround => todo!(),
            JustifyContent::SpaceEvenly => todo!(),
        };
        
        // Computes the growth total of each node
        let grow_total = nodes
            .iter()
            .map(|n| n.style.config.grow.max(0.0))
            .sum::<f32>()
            .max(1.0);

        let mut pos = starting_pos;
        for node in nodes {
            let grow_perc = node.style.config.grow / grow_total;
            let node_width = node.cache.basis + width_remaining * grow_perc;
            let node_height = parent_size.y;
            let node_size = Vec2::newa::<A>(node_width, node_height);
            let region = Rect::new(pos, node_size);
            node.cache.region = region;
            pos += Vec2::newa::<A>(region.size.x + space_between, 0.0);
        }
        let grow_total = grow_total.max(1.0);
    }

    unsafe fn get_nodes<'a>(&mut self, nodes: &mut Vec<&'a mut Node>, node_ids: &[NodeId]) {
        nodes.clear();
        for node_id in node_ids {
            let node = self.get_mut(*node_id).unwrap();
            let node = std::mem::transmute(node);
            nodes.push(node);
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