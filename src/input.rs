use crate::{Gewy, Result, GewyEnterEvent, GewyExitEvent, PressEvent, ReleaseEvent, EnterEvent, ExitEvent, NodeId};
use glam::Vec2;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub(crate) struct Cursor {
    pub position: Vec2,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub pressed_node: Option<NodeId>,
    pub icon: CursorIcon
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton { Left, Right }

/// Different cursor icons, shamelessly copied from winit.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum CursorIcon {
    #[default]
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize
}

/**
 * API for mapping the external window manager to a [`Gewy`] and vice versa.
 */
pub struct InputMapping<'a> {
    pub(crate) gewy: &'a mut Gewy
}
impl<'a> InputMapping<'a> {

    /// Alerts the [`Gewy`] that the cursor has entered window containing it.
    /// Fires relevant events.
    pub fn enter_cursor(&mut self) -> Result<()> {
        self.gewy.fire_global(GewyEnterEvent)
    }

    /// Alerts the [`Gewy`] that the cursor has exited window containing it.
    /// Fires relevant events.
    pub fn exit_cursor(&mut self) -> Result<()> {
        self.gewy.fire_global(GewyExitEvent)?;
        if let Some(pressed_node) = self.gewy.cursor.pressed_node {
            self.gewy.fire_bubble(ReleaseEvent, pressed_node)?;
            self.gewy.cursor.pressed_node = None;
        }
        Ok(())
    }

    /// Moves the [`Gewy`]'s internal cursor.
    /// Fires relevant events.
    pub fn move_cursor(&mut self, position: Vec2) -> Result<()> {
        let touching_id = self.gewy.get_touching_id(position);
        let prev_touching_id = self.gewy.get_touching_id(self.gewy.cursor.position);
        match (touching_id, prev_touching_id) {
            (None, Some(prev_id)) => self.gewy.fire_bubble(ExitEvent, prev_id)?,
            (Some(node_id), None) => self.gewy.fire_bubble(EnterEvent, node_id)?,
            (Some(node_id), Some(prev_id)) if node_id != prev_id => {
                self.gewy.fire_bubble(ExitEvent, prev_id)?;
                self.gewy.fire_bubble(EnterEvent, node_id)?;

            },
            _ => {}
        }        
        self.gewy.cursor.position = position;
        Ok(())
    }

    /// Simulates a touch or a click on the [`Gewy`] at the current position of the internal cursor.
    /// Fires relevant events.
    pub fn press(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => {
                self.gewy.cursor.left_pressed = true;
                self.gewy.fire_bubble_at(PressEvent, self.gewy.cursor.position)?;
            },
            MouseButton::Right => {
                self.gewy.cursor.right_pressed = true;
                self.gewy.fire_bubble_at(PressEvent, self.gewy.cursor.position)?;
            }
        }
        Ok(())
    }

    /// Simulates the release of a touch or click on the [`Gewy`] at the current position of the internal cursor.
    /// Fires relevant events.
    pub fn release(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => {
                self.gewy.cursor.left_pressed = false;
                let Some(pressed_id) = self.gewy.pressed_id else { return Ok(()) };
                self.gewy.pressed_id = None;
                let Some(node_touching_id) = self.gewy.get_touching_id(self.gewy.cursor.position) else { return Ok(()) };
                if node_touching_id != pressed_id { return Ok(()) }
                self.gewy.fire_bubble(ReleaseEvent, pressed_id)?;
            },
            MouseButton::Right => {
                self.gewy.cursor.right_pressed = false;
                self.gewy.fire_global(ReleaseEvent)?;
            }
        }
        Ok(())
    }

    /// Takes any updates the the internal cursor icon.
    pub fn take_cursor_icon(&mut self) -> Option<CursorIcon> {
        std::mem::take(&mut self.gewy.next_cursor_icon)
    }
}
