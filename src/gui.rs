use glam::Vec2;
use slotmap::SlotMap;
use crate::{GUiError, Result, NodeChildren, Node, NodeId, Rect, Layout, JustifyContent, extensions::VecExtensions, Painter, AlignItems};

/// Represents a graphical user interface, and a torage of [`Node`]s.
#[derive(Debug, Default)]
pub struct Gui {
    storage: SlotMap<NodeId, Node>,
    root_id: NodeId,
    pub translation: Vec2,
    pub scale: f32,
    pub round: bool
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
            scale: 1.0,
            round: true
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

        // Stores node as a child of another.
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
            let children = NodeChildren { gui, parent_id: node_id };
            let node = self.storage.get_mut(node_id).unwrap();
            node.widget.children(children)?;
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
        let region = node.raw.region;
        self.compute_regions(children, region, layout);
    }

    fn compute_regions(
        &mut self,
        node_ids: &[NodeId],
        parent_region: Rect,
        parent_layout: Layout
    ) {
        let is_reverse = parent_layout.direction.is_reverse();
        let is_row = parent_layout.direction.is_row();
        let parent_size = parent_region.size.flip(!is_row);

        // Packs nodes horizontally, and aligns them vertially.
        let (raw_basis_total, raw_group_width) = self.pack_group(node_ids, parent_size, is_row, is_reverse);
        self.align_group(node_ids, parent_size.y, parent_size.y, parent_layout.align_items);

        // Either grows or shinks on primary axis.
        if raw_basis_total <= parent_size.x {
            self.grow_group(node_ids, raw_group_width, parent_size.x, parent_layout.justify_content, is_reverse);
        }
        else {
            log::warn!("Shrinking not yet supported");
            return;
        }

        // Moves children from their local coordinate space to the global one.
        // Round their positions if configured to do so.
        for id in node_ids {
            let node = self.get_mut(*id).unwrap();
            node.raw.region = node.raw.region.flip(!is_row);
            node.raw.region.pos += parent_region.pos;
            self.compute_region(*id);
        }
    }

    fn pack_group(&mut self, group: &[NodeId], parent_size: Vec2, is_row: bool, is_reverse: bool) -> (f32, f32) {

        // Computes "basis" and "grow" totals.
        // Converts various properties to their "raw" values for later usage.
        let mut raw_basis_total = 0.0;
        let mut grow_total = 0.0;
        for id in group {
            
            let node = self.get_mut(*id).unwrap();
            node.raw.basis = node.style.raw_basis(parent_size.x, is_row);
            node.raw.corners = node.style.raw_corners(parent_size);

            raw_basis_total += node.raw.basis;
            grow_total += node.style.config.grow.max(0.0);
        }
        grow_total = grow_total.max(1.0);

        // Resolves the raw size of each node and packs them closely together, left to right.
        // Computes total raw width.
        let remaining_width_basis = parent_size.x - raw_basis_total;
        let mut raw_group_width = 0.0;
        let mut group_height = 0.0;
        let node_ids = group.iter();
        for_each(node_ids, is_reverse, |id| {
            let node = self.get_mut(*id).unwrap();
            let grow_perc = node.style.config.grow.max(0.0) / grow_total;
            let raw_width = node.raw.basis + grow_perc * remaining_width_basis;
            let raw_height = node.style.raw_height(parent_size.y, is_row);
            if raw_height > group_height {
                group_height = raw_height;
            }
            node.raw.region = Rect {
                pos: Vec2::new(raw_group_width, 0.0),
                size: Vec2::new(raw_width, raw_height)
            };
            raw_group_width += raw_width;
        });

        // Done
        (raw_basis_total, raw_group_width)
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
            node.raw.region.pos.x += offset + spacing_accum;
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
                    node.raw.region.size.y = group_height;
                },
                AlignItems::Center => {
                    let node_size = node.raw.region.size;
                    let node_height = node_size.y;
                    node.raw.region.pos.y = parent_height / 2.0 - node_height / 2.0;
                },
                AlignItems::End =>  {
                    let node_height = node.raw.region.size.y;
                    node.raw.region.pos.y = parent_height - node_height;
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
        painter.translation = node.raw.region.pos;
        if self.round {
            painter.translation = painter.translation.round();
        }
        widget.paint(style, node.raw.canvas(), painter);

        // Renders children of node
        let children: &[NodeId] = unsafe {
            std::mem::transmute(node.children())
        };
        for child_id in children {
            self.paint_node(*child_id, painter);
        }
    }
}


/// Helper function that allows for iterating either forwards backwards based on a boolean flag.
/// I hate this :(
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
    use crate::{Gui, Node, NodeId};

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
        NodeId::default();
        
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