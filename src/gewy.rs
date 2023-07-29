use std::collections::HashMap;

use glam::Vec2;
use slotmap::SlotMap;
use tinyvec::TinyVec;
use crate::*;
use crate::extensions::VecExtensions;
use crate::util::SliceIter;

const EPS: f32 = 0.001;
type NodeIdVec = TinyVec<[NodeId; 32]>;

/// Represents a graphical user interface, and a torage of [`Node`]s.
#[derive(Default)]
pub struct Gewy {
    storage: SlotMap<NodeId, Node>,
    root_id: NodeId,
    named_index: HashMap<Name, Vec<NodeId>>,
    pub(crate) pressed_id: Option<NodeId>,
    pub(crate) cursor: Cursor,
    pub(crate) next_cursor_icon: Option<CursorIcon>,
    pub translation: Vec2,
    pub scale: f32,
    pub round: bool
}

impl Gewy {
    /// Creates a new [`Gewy`] instance alongside the id of the root node.
    pub fn new(root_node: Node) -> Self {
        let mut storage = SlotMap::<NodeId, Node>::default();
        let root_id = storage.insert(root_node);
        let mut slf = Self {
            storage,
            root_id,
            named_index: HashMap::new(),
            pressed_id: None,
            cursor: Cursor::default(),
            next_cursor_icon: None,
            translation: Vec2::ZERO,
            scale: 1.0,
            round: true
        };
        unsafe { slf.spawn_descendants(root_id) };
        slf
    }

    pub fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Id of the root node.
    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    /// Inserts as a child of another.
    /// Returns id of node inserted.
    pub fn insert(&mut self, parent_id: NodeId, mut node: Node) -> Result<NodeId> {

        // Stores node as a child of another.
        node.parent_id = Some(parent_id);
        let node_id = self.storage.insert(node);
        let Some(parent) = self.storage.get_mut(parent_id) else {
            self.storage.remove(node_id);
            return Err(GewyError::ParentNodeNotFound.into())
        };
        parent.children_ids.push(node_id);

        // Spawns descendants of node's widget
        unsafe { self.spawn_descendants(node_id) };

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

    pub fn iter_named<'a>(&'a mut self, name: Name) -> impl Iterator<Item = &'a mut Node> + '_ {
        let node_ids: &[NodeId] = unsafe { std::mem::transmute(self.ids_with_name(name)) };
        NodeIteratorMut {
            gewy: self,
            node_ids,
            index: 0,
        }
    }

    pub fn ids_with_name(&mut self, name: Name) -> &[NodeId] {
        let ids = self.named_index.entry(name).or_insert_with(|| Vec::new());
        &ids[..]
    }

    pub fn get(&self, node_id: NodeId) -> Result<&Node> {
        self.storage.get(node_id).ok_or(GewyError::NodeNotFound)
    }
    
    pub fn get_mut(&mut self, node_id: NodeId) -> Result<&mut Node> {
        self.storage.get_mut(node_id).ok_or(GewyError::NodeNotFound)
    }

    /// Gets the id of the node touching the position specified.
    pub fn get_touching_id(&self, position: Vec2) -> Option<NodeId> {
        self.node_touching(self.root_id, position)
    }

    /// Gets the node touching the position specified.
    pub fn get_touching(&self, position: Vec2) -> Option<&Node> {
        self
            .node_touching(self.root_id, position)
            .map(|id| unsafe { self.get_unsafe(id).unwrap() } )
    }

    /// Gets the node touching the position specified.
    pub fn get_touching_mut(&mut self, position: Vec2) -> Option<&mut Node> {
        self
            .node_touching(self.root_id, position)
            .map(|id| unsafe { self.get_mut_unsafe(id).unwrap() } )
    }

    // Gets ID of node touching 
    fn node_touching<'a>(&'a self, node_id: NodeId, cursor: Vec2) -> Option<NodeId> {
        let node: &Node = unsafe { self.get_unsafe(node_id).unwrap() };
        for child_id in node.children_ids.iter().rev() {
            if let Some(id) = self.node_touching(*child_id, cursor) {
                return Some(id);
            }
        }
        if node.raw.padding_region().contains(cursor) {
            Some(node_id)
        }
        else {
            None
        }
    }

    pub unsafe fn get_unsafe<'a>(&self, node_id: NodeId) -> Result<&'a Node> {
        let node = self.storage.get(node_id).ok_or(GewyError::NodeNotFound);
        std::mem::transmute(node)
    }

    pub unsafe fn get_mut_unsafe<'a>(&mut self, node_id: NodeId) -> Result<&'a mut Node> {
        let node = self.storage.get_mut(node_id).ok_or(GewyError::NodeNotFound);
        std::mem::transmute(node)
    }

    /// Fires an event on whatever node is touching the point, and bubbles that event up to the root node.
    pub fn fire_bubble_at(&mut self, event: impl Event, point: Vec2) -> Result<()> {
        let Some(node_id) = self.node_touching(self.root_id, point) else {
            return Ok(())
        };
        self.fire_bubble(event, node_id)?;
        Ok(())
    }

    /// Fires an event on a specific node, and bubbles that event up to the root node.
    pub fn fire_bubble<'a>(&'a mut self, event: impl Into<DynEvent>, mut node_id: NodeId) -> Result<()> {
        let event: DynEvent = event.into();
        let mut node: &mut Node = unsafe { self.get_mut_unsafe(node_id)? };
        let mut ctl = EventControl::new(&event, None);
        loop {
            
            // Has Widget of current node handle event.
            let style = &mut node.style;
            let mut children = Descendants::new(node_id, self);
            node.widget.event(style, &mut children, &mut ctl)?;
            let stop_propagation = ctl.stop;

            // Sets cursor
            if let Some(icon) = ctl.cursor_icon {
                self.next_cursor_icon = Some(icon);
            }
        
            // Fires outgoing events.
            let Some(ancestor_id) = node.ancestor_id else { break };
            Self::fire_outgoing_events(self, ctl, node_id, ancestor_id)?;
            
            // Bubbles up.
            if stop_propagation { break }
            ctl = EventControl::new(
                &event,
                Some(NodeOrigin {
                    id: node_id,
                    name: node.name
                })
            );
            node_id = ancestor_id;
            node = unsafe { self.get_mut_unsafe(node_id).unwrap() };        
        }
        Ok(())
    }

    // Fires an event globally.
    pub fn fire_global(&mut self, event: impl Event) -> Result<()> {
        let event = DynEvent::new(event);
        let storage: &mut SlotMap<NodeId, Node> = unsafe { std::mem::transmute(&mut self.storage) };
        for (node_id, node) in storage.iter_mut() {

            // Has widget of current node handle event.
            let mut children = Descendants { ancestor_id: node_id, parent_id: node_id, gewy: self };
            let mut ctl = EventControl::new(&event, None);
            node.widget.event(&mut node.style, &mut children, &mut ctl)?;

            // Sets cursor
            if let Some(icon) = ctl.cursor_icon {
                self.next_cursor_icon = Some(icon);
            }

            // Handles output of EventControl.
            let Some(ancestor_id) = node.ancestor_id else { break };
            Self::fire_outgoing_events(self, ctl, node_id, ancestor_id)?;
        }
        Ok(())
    }

    pub fn resize(&mut self, size: Vec2) {
        self.layout_children(
            &[self.root_id],
            Rect::new(Vec2::ZERO, size),
            Direction::default(),
            Justify::default(),
            Align::default()
        );
    }


    /// Provides object for mapping external inputs to internal events.
    pub fn mapping(&mut self) -> InputMapping<'_> {
        InputMapping { gewy: self }
    }

    /// Paints the ui using the supplied painter.
    pub fn paint(&mut self, painter: &mut Painter) {
        self.paint_node(self.root_id, painter);
    }

    // Spawns descendants of the node specified using its widget.
    // Assumes the node has been spawned, but has no children.
    unsafe fn spawn_descendants(&mut self, node_id: NodeId) {
        let gewy = self as *mut Self;
        let gewy = &mut *gewy;
        let node = self.storage.get_mut(node_id).unwrap();
        node.widget.descendants(&mut Descendants::new(node_id, gewy));
    }

    // Computes the raw regions of this node's children.
    fn layout_children_of<'a>(&'a mut self, node_id: NodeId) {
        
        // Gets children of node
        let node = self.get(node_id).unwrap();
        let children: &[NodeId] = unsafe { std::mem::transmute(node.children()) };
        if children.is_empty() { return }

        // Computes regions of children
        let region = node.raw.content_region();
        self.layout_children(children, region, node.style.direction, node.style.justify, node.style.align);
    }

    // Computes the raw regions of a group of nodes given their parent's raw region.
    fn layout_children(
        &mut self,
        child_ids: &[NodeId],
        parent_region: Rect,
        parent_direction: Direction,
        parent_justify: Justify,
        parent_align: Align
    ) {
        // Unpack parent metadata.
        let is_reverse = parent_direction.is_reverse();
        let is_row = parent_direction.is_row();
        let parent_size = parent_region.size.flip(!is_row);

        // Computes raw values for nodes, and prepares them for further layout code. Accumulates sums.
        let (group_basis_width, group_content_width, grow_total, shrink_total) = self.prepare_group(
            child_ids,
            parent_size,
            is_row,
            is_reverse
        );  
        
        // Either grows or shrinks
        let group_final_width = if group_basis_width <= parent_size.x {
            self.grow_group(child_ids, group_basis_width, grow_total, parent_size.x, is_reverse)
        }
        else {
            self.shrink_group(child_ids, group_basis_width, group_content_width, shrink_total, parent_size.x, is_reverse)
        };

        // Justifies and aligns children
        self.justify_group(child_ids, group_final_width, parent_size.x, parent_justify, is_reverse);
        self.align_group(child_ids, parent_size.y, parent_size.y, parent_align);

        // Transforms children to the global coordinate space.
        for child_id in child_ids {
            let node = self.get_mut(*child_id).unwrap();
            node.raw.region = node.raw.region.flip(!is_row);
            node.raw.region.position += parent_region.position;
            node.raw.margin = node.raw.margin.rotate_left(!is_row);
            node.raw.padding = node.raw.padding.rotate_left(!is_row);
            if !node.children().is_empty() {
                self.layout_children_of(*child_id);
            }
        }
    }

    // Computes raw values for each node.
    // Sets each node's size to their "basis size".
    // Returns various sums for later.
    fn prepare_group(&mut self, group: &[NodeId], parent_size: Vec2, is_row: bool, is_reverse: bool) -> (f32, f32, f32, f32) {
        let mut group_full_basis_width = 0.0;   // Total width of group (basis + margin + padding) if they were stacked at their basis size side-by-side
        let mut group_basis_width = 0.0;        // Total width of group (basis ) if they were stacked at their basis size side-by-side
        let mut grow_total = 0.0;               // Sum of "grow" values, to be used during shrinking.
        let mut shrink_total = 0.0;             // Sum of "shrink" values, to be used during growing.
        for id in SliceIter::new(group, is_reverse) {
            
            // Calculates raw sizes
            let node = self.get_mut(*id).unwrap();
            let min_size = node.style.raw_min_size(parent_size, is_row);
            let max_size = node.style.raw_max_size(parent_size, is_row);
            let padding_region_size = node.raw.padding_region_size();
            node.raw.margin = node.style.raw_margin(parent_size, is_row);
            node.raw.padding = node.style.raw_padding(parent_size, is_row);
            node.raw.min_size = min_size;
            node.raw.max_size = max_size.max(min_size);
            node.raw.corners = node.style.raw_corners(padding_region_size);

            // Sets initial size of node
            let basis_size = node.style.raw_basis(parent_size.x, is_row);
            let height = node.style.raw_height(parent_size.y, is_row);
            node.raw.set_size(Vec2::new(basis_size, height));

            // Accumulates sums
            group_basis_width += basis_size;
            group_full_basis_width += node.raw.full_width();
            grow_total += node.style.grow;
            shrink_total += node.style.shrink;
        };
        grow_total = grow_total.max(1.0);

        // Returns sums
        (group_full_basis_width, group_basis_width, grow_total, shrink_total)
    }

    // Grows "packed" children on the primary axis.
    fn grow_group(
        &mut self,
        group: &[NodeId],
        group_width: f32,
        grow_total: f32,
        parent_width: f32,
        is_reverse: bool
    ) -> f32 {

        // Setup
        let grow_width = parent_width - group_width;
        let mut capped_width = 0.0;
        let mut uncapped_width = 0.0;
        let mut uncapped_grow_total = 0.0;
        let mut uncapped_group = NodeIdVec::new();

        // Computes the "capped" and "uncapped" group values.
        for id in SliceIter::new(group, is_reverse) {
            let node = self.get_mut(*id).unwrap();
            let grow = node.style.grow;
            let grow_ratio = grow.max(0.0) / grow_total;
            let max_width = node.raw.max_size.x;
            let grown_width = node.raw.width() + grow_width * grow_ratio;
            let is_capped = grown_width > max_width;
            if is_capped {
                node.raw.set_width(max_width);
                capped_width += node.raw.region.size.x;
            }
            else {
                node.raw.set_width(grown_width);
                uncapped_width += node.raw.full_width();
                uncapped_grow_total += grow;
                uncapped_group.push(*id);
            };
        };

        // Return width if none of the nodes were "capped".
        if uncapped_group.len() == group.len() {
            return uncapped_width;
        }
        // Otherwise, recurse.
        capped_width + self.grow_group(
            &uncapped_group,
            uncapped_width,
            uncapped_grow_total.max(1.0),
            parent_width - capped_width,
            false
        )
    }

    // Shrinks "packed" children on the primary axis.
    fn shrink_group(
        &mut self,
        group: &[NodeId],
        group_full_width: f32,
        group_width: f32,
        shrink_total: f32,
        parent_width: f32,
        is_reverse: bool
    ) -> f32 {

        // Calculates "scaled shave".
        let group_shave = group_full_width - parent_width;
        let mut scaled_group_shave = 0.0;
        for id in SliceIter::new(group, is_reverse) {
            let node = self.get(*id).unwrap();
            let width = node.raw.width();
            let shrink = node.style.shrink;
            let shrink_ratio = shrink / shrink_total;
            let width_ratio = width / group_width;
            let scaled_shave = group_shave * width_ratio * shrink_ratio;
            scaled_group_shave += scaled_shave;
        };
        if scaled_group_shave < EPS {
            return group_full_width;
        }

        // Shaves off as much from the each node as possible and positions them side-by-side.
        let shave_ratio = group_shave / scaled_group_shave;
        let mut capped_full_width = 0.0;
        let mut uncapped_full_width = 0.0;
        let mut uncapped_width = 0.0;
        let mut uncapped_shrink_total = 0.0;
        let mut uncapped_group = NodeIdVec::new();
        for id in SliceIter::new(group, is_reverse) {
            let node = self.get_mut(*id).unwrap();
            let width = node.raw.width();
            let shrink = node.style.shrink;
            let shrink_ratio = shrink / shrink_total.max(1.0);
            let width_ratio = width / group_width;
            let scaled_shave = group_shave * width_ratio * shrink_ratio * shave_ratio;
            let shrunk_width = width - scaled_shave;
            let min_width = node.raw.min_size.x;
            let is_capped = shrunk_width <= min_width;
            if is_capped {
                node.raw.set_width(min_width);
                capped_full_width += node.raw.full_width();
            }
            else {
                node.raw.set_width(shrunk_width);
                uncapped_width += shrunk_width;
                uncapped_full_width += node.raw.full_width();
                uncapped_shrink_total += shrink;
                uncapped_group.push(*id);
            }
        };

        // If still too big and there is at least some left to shave off, repeat.
        if uncapped_group.len() == group.len() {
            uncapped_full_width
        }
        else {
            capped_full_width + self.shrink_group(
                &uncapped_group,
                uncapped_full_width,
                uncapped_width,
                uncapped_shrink_total,
                parent_width - capped_full_width,
                is_reverse
            )
        }
    }

    fn justify_group(
        &mut self,
        group: &[NodeId],
        group_width: f32,
        parent_width: f32,
        justify_content: Justify,
        is_reverse: bool
    ) {
        // Determines offset and spacing of nodes based on the layout.
        let (offset, spacing) = match justify_content {
            Justify::Start => (0.0, 0.0),
            Justify::End => (parent_width - group_width, 0.0),
            Justify::Center => (parent_width/2.0 - group_width/2.0, 0.0),
            Justify::SpaceBetween => {
                let remaining_width = parent_width - group_width;
                let num_gaps = (group.len() - 1) as f32;
                (0.0, remaining_width / num_gaps)
            },
            Justify::SpaceAround => {
                let remaining_width = parent_width - group_width;
                let num_nodes = group.len() as f32;
                let gap = remaining_width / num_nodes;
                (gap / 2.0, gap)
            },
            Justify::SpaceEvenly => {
                let remaining_width = parent_width - group_width;
                let num_gaps = (group.len() + 1) as f32;
                let gap = remaining_width / num_gaps;
                (gap, gap)
            }
        };

        // Applies offset and spacing to nodes.
        let mut x = offset;
        let group_ids = SliceIter::new(group, is_reverse);
        for id in group_ids {
            let node = self.get_mut(*id).unwrap();
            let node_width = node.raw.region.size.x;
            node.raw.region.position.x = x;
            x += node_width + spacing;
        };
    }


    // Aligns children on the secondary axis.
    fn align_group(
        &mut self,
        group: &[NodeId],
        group_height: f32,
        parent_height: f32,
        parent_align_items: Align
    ) {
        for id in group {
            let node = self.get_mut(*id).unwrap();
            let node_align_self = node.style.align_self;
            let node_align = node_align_self.to_align_items(parent_align_items);
            let node_height = node.style.height;
            match node_align {
                Align::Center => {
                    let node_size = node.raw.region.size;
                    let node_height = node_size.y;
                    node.raw.region.position.y = parent_height / 2.0 - node_height / 2.0;
                },
                Align::Stretch if node_height == Val::Auto => {
                    node.raw.region.size.y = group_height
                },
                Align::End =>  {
                    let node_height = node.raw.region.size.y;
                    node.raw.region.position.y = parent_height - node_height;
                }
                _ => {}
            }
        }
    }

    fn fire_outgoing_events(&mut self, ctl: EventControl, node_id: NodeId, ancestor_id: NodeId) -> Result<()> {
        if ctl.pressed {
            self.pressed_id = Some(node_id);
        }
        for out_event in ctl.outgoing_events {
            self.fire_bubble(out_event, ancestor_id)?;
        }
        Ok(())
    }

    fn paint_node(&mut self, node_id: NodeId, painter: &mut Painter) {
        
        // Unpacks node
        let node = self.get(node_id).unwrap();
        let widget = &node.widget;
        let style = &node.style;

        // Gets padding region
        let mut paint_region = node.raw.padding_region();
        let mut corners = node.raw.corners;
        if self.round {
            let unit = 1.0/self.scale;
            paint_region = paint_region.round(unit);
            corners = corners.round(unit);
        }

        // Paints widget in the padding region
        let paint_size = paint_region.size;
        if paint_size.x > EPS && paint_size.y > EPS {
            let canvas = Canvas {
                size: paint_size,
                corners
            };
            let state = painter.push_state();
            painter.translation = paint_region.position;
            widget.paint(style, painter, canvas);
            painter.pop_state(state);
        }

        // Renders children of node
        let children: &[NodeId] = unsafe { std::mem::transmute(node.children()) };
        for child_id in children {
            self.paint_node(*child_id, painter);
        }
    }
}

pub struct NodeIteratorMut<'a> {
    gewy: &'a mut Gewy,
    node_ids: &'a [NodeId],
    index: usize
}

impl<'a> Iterator for NodeIteratorMut<'a> {
    type Item = &'a mut Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.node_ids.len() {
            return None;
        }
        let node_id = self.node_ids[self.index];
        self.index += 1;
        unsafe {
            let node = self.gewy.get_mut(node_id).unwrap();
            Some(std::mem::transmute(node))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Gewy, Node, NodeId};

    #[test]
    fn test_insert() {
        let mut gewy = Gewy::new(Node::default());
        let root_id = gewy.root_id;
        let child_1_id = gewy.insert(root_id, Node::default()).unwrap();
        let child_2_id = gewy.insert(root_id, Node::default()).unwrap();
        
        let root = gewy.get(root_id).unwrap();
        assert_eq!(2, root.children().len());

        let child_1 = gewy.get(child_1_id).unwrap();
        assert_eq!(0, child_1.children().len());
        assert_eq!(Some(root_id), child_1.parent());

        let child_2 = gewy.get(child_2_id).unwrap();
        assert_eq!(0, child_2.children().len());
        assert_eq!(Some(root_id), child_2.parent());
    }

    #[test]
    fn test_remove() {
        let mut gewy = Gewy::new(Node::default());
        let root_id = gewy.root_id;
        let child_1_id = gewy.insert(root_id, Node::default()).unwrap();
        let child_2_id = gewy.insert(root_id, Node::default()).unwrap();
        NodeId::default();
        
        let root = gewy.get(root_id).unwrap();
        assert_eq!(2, root.children().len());

        gewy.remove(child_1_id).unwrap();
        let root = gewy.get(root_id).unwrap();
        assert_eq!(1, root.children().len());

        gewy.remove(child_2_id).unwrap();
        let root = gewy.get(root_id).unwrap();
        assert_eq!(0, root.children().len());
        
        assert!(gewy.remove(root_id).is_none());
    }
}