use glam::Vec2;
use slotmap::SlotMap;
use crate::{GUiError, Result, NodeChildren, Node, NodeId, NodePath, NodePathElem, Rect, Layout, JustifyContent, extensions::VecExtensions, Painter, AlignItems};

/// Represents a graphical user interface, and a torage of [`Node`]s.
#[derive(Debug, Default)]
pub struct Gui {
    storage: SlotMap<NodeId, Node>,
    root_id: NodeId,
    pub translation: Vec2,
    pub scale: f32
}

impl Gui {

    /// Creates a new [`Gui`] instance alongside the id of the root node.
    pub fn new(root: Node) -> (NodeId, Self) {
        let mut storage = SlotMap::<NodeId, Node>::default();
        let root_id = storage.insert(root);
        let slf = Self {
            storage,
            root_id,
            translation: Vec2::ZERO,
            scale: 1.0
        };
        (root_id, slf)
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
        let layout = Layout {
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        self.compute_regions(
            &[self.root_id],
            Rect::new(Vec2::ZERO, size),
            layout
        );
    }

    fn compute_region<'a>(&'a mut self, node_id: NodeId) {
        
        // Gets children of node
        let node = self.get(node_id).unwrap();
        let children: &[NodeId] = unsafe { std::mem::transmute(node.children()) };
        if children.is_empty() { return }

        // Computes regions of children
        let layout = node.style.layout;
        let region = node.cache.region;
        self.compute_regions(children, region, layout);
    }

    fn compute_regions(
        &mut self,
        node_ids: &[NodeId],
        parent_region: Rect,
        parent_layout: Layout
    ) {
        // Computes basis of each node, and returns the total
        let is_row: bool = parent_layout.direction.is_row();
        let parent_size = parent_region.size.flip(!is_row);
        let mut basis_total = 0.0;
        for id in node_ids {
            let node = self.get_mut(*id).unwrap();
            let basis = node.style.get_basis(parent_size.x, is_row);
            node.cache.basis = basis;
            basis_total += basis;
        }
        let is_reverse = parent_layout.direction.is_reverse();

        // Either grows or shinks on primary axis
        if basis_total <= parent_size.x {
            let group_size = self.pack_group(node_ids, parent_size, basis_total, is_row, is_reverse);
            self.grow_group(node_ids, group_size.x, parent_size.x, parent_layout.justify_content, is_reverse);
            self.align_group(node_ids, parent_size.y, parent_size.y, parent_layout.align_items);
        }
        else {
            log::warn!("Shrinking not yet supported");
            return;
        }

        // Moves children from their local coordinate space to a global one.
        for id in node_ids {
            let node = self.get_mut(*id).unwrap();
            node.cache.region = node.cache.region.flip(!is_row);
            node.cache.region.pos += parent_region.pos;
            self.compute_region(*id);
        }
    }

    // Computes the "effective size" of each node.
    // Accumulates the total "effective size".
    // Packs nodes next to each other contiguously as an intermediate step.
    fn pack_group(&mut self, group: &[NodeId], parent_size: Vec2, basis_total: f32, is_row: bool, is_reverse: bool) -> Vec2 {

        // Computes the total "grow" value of all nodes.
        let mut grow_total = 0.0;
        for id in group {
            let node = self.get_mut(*id).unwrap();
            grow_total += node.style.config.grow.max(0.0);
        }
        grow_total = grow_total.max(1.0);

        // Resolves the pixel size of each node and packs them closely together.
        let remaining_width_basis = parent_size.x - basis_total;
        let mut group_width = 0.0;
        let mut group_height = 0.0;
        let node_ids = group.iter();
        for_each(node_ids, is_reverse, |id| {
            let node = self.get_mut(*id).unwrap();
            let grow_perc = node.style.config.grow / grow_total;
            let node_width = node.cache.basis + grow_perc * remaining_width_basis;
            let node_height = node.style.get_effective_height(parent_size.y, is_row);
            if node_height > group_height {
                group_height = node_height;
            }
            node.cache.region = Rect {
                pos: Vec2::new(group_width, 0.0),
                size: Vec2::new(node_width, node_height)
            };
            group_width += node_width;
        });

        Vec2::new(group_width, group_height)
    }

    // Grows "packed" children on the primary axis.
    fn grow_group(
        &mut self,
        group: &[NodeId],
        group_width: f32,
        parent_width: f32,
        justify_content: JustifyContent,
        is_reverse: bool
    ) {

        // Determines offset and spacing of nodes based on the layout.    
        let (offset, spacing) = match justify_content {
            JustifyContent::Start => return,
            JustifyContent::End => (parent_width - group_width, 0.0),
            JustifyContent::Center => (parent_width/2.0 - group_width/2.0, 0.0),
            JustifyContent::SpaceBetween => {
                let remaining_width = parent_width - group_width;
                let num_gaps = (group.len() - 1) as f32;
                (0.0, remaining_width / num_gaps)
            },
            JustifyContent::SpaceAround => {
                let remaining_width = parent_width - group_width;
                let num_nodes = group.len() as f32;
                let gap = remaining_width / num_nodes;
                (gap / 2.0, gap)
            },
            JustifyContent::SpaceEvenly => {
                let remaining_width = parent_width - group_width;
                let num_gaps = (group.len() + 1) as f32;
                let gap = remaining_width / num_gaps;
                (gap, gap)
            }
        };

        // Applies offset and spacing to nodes.
        let id_iter = group.iter();
        let mut spacing_accum = 0.0;
        for_each(id_iter, is_reverse, |id| {
            let node = self.get_mut(*id).unwrap();
            node.cache.region.pos.x += offset + spacing_accum;
            spacing_accum += spacing;
        });
    }

    // Aligns "packed" children on the secondary axis.
    fn align_group(
        &mut self,
        group: &[NodeId],
        group_height: f32,
        parent_height: f32,
        align_items: AlignItems
    ) {

        for id in group {
            let node = self.get_mut(*id).unwrap();
            let align_items = node.style.config.align_self.resolve(align_items);
            match align_items {
                AlignItems::Start => {},
                AlignItems::Stretch => {
                    node.cache.region.size.y = group_height;
                },
                AlignItems::Center => {
                    let node_size = node.cache.region.size;
                    let node_height = node_size.y;
                    node.cache.region.pos.y = parent_height / 2.0 - node_height / 2.0;
                },
                AlignItems::End =>  {
                    let node_height = node.cache.region.size.y;
                    node.cache.region.pos.y = parent_height - node_height;
                }
            }
        }
    }

    pub fn paint(&mut self, painter: &mut Painter) {
        self.paint_node(self.root_id, painter);
    }

    fn paint_node(&mut self, node_id: NodeId, painter: &mut Painter) {
        
        // Unpacks node
        let node = self.get(node_id).unwrap();
        let widget = &node.widget;
        let style = &node.style;

        // Paints widget
        painter.translation = node.cache.region.pos;
        widget.render_self(style, node.cache.region.size, painter);

        // Renders children of node
        let children: &[NodeId] = unsafe {
            std::mem::transmute(node.children())
        };
        for child_id in children {
            self.paint_node(*child_id, painter);
        }
    }
}


/// Helper function that allows for either iterating forwards or in reverse based on a boolean flag.
fn for_each<I, T, F>(iter: I, reverse: bool, mut f: F)
where
    I: DoubleEndedIterator<Item = T>,
    F: FnMut(T)
{
    if reverse {
        for elem in iter.into_iter().rev() {
            f(elem)
        }
    }
    else {
        for elem in iter {
            f(elem)
        }
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