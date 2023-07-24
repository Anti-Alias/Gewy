use crate::{Gui, Result, GuiEnterEvent, GuiExitEvent, PressEvent, ReleaseEvent, EnterEvent, ExitEvent, NodeId};
use glam::Vec2;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub(crate) struct Cursor {
    position: Vec2,
    left_pressed: bool,
    right_pressed: bool,
    pressed_node: Option<NodeId>
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton { Left, Right }

/**
 * Contains API for mapping external inputs to internal events.
 */
pub struct InputMapping<'a> {
    pub(crate) gui: &'a mut Gui
}
impl<'a> InputMapping<'a> {

    pub fn enter_cursor(&mut self) -> Result<()> {
        self.gui.fire_global(GuiEnterEvent)
    }

    pub fn exit_cursor(&mut self) -> Result<()> {
        self.gui.fire_global(GuiExitEvent)?;
        if let Some(pressed_node) = self.gui.cursor.pressed_node {
            self.gui.fire_bubble(ReleaseEvent, pressed_node)?;
            self.gui.cursor.pressed_node = None;
        }
        Ok(())
    }

    pub fn move_cursor(&mut self, position: Vec2) -> Result<()> {
        let touching_id = self.gui.get_touching_id(position);
        let prev_touching_id = self.gui.get_touching_id(self.gui.cursor.position);
        match (touching_id, prev_touching_id) {
            (None, Some(prev_id)) => self.gui.fire_bubble(ExitEvent, prev_id)?,
            (Some(node_id), None) => self.gui.fire_bubble(EnterEvent, node_id)?,
            (Some(node_id), Some(prev_id)) if node_id != prev_id => {
                self.gui.fire_bubble(EnterEvent, node_id)?;
                self.gui.fire_bubble(ExitEvent, prev_id)?;
            },
            _ => {}
        }        
        self.gui.cursor.position = position;
        Ok(())
    }

    pub fn press(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => {
                self.gui.cursor.left_pressed = true;
                self.gui.fire_bubble_at(PressEvent, self.gui.cursor.position)?;
            },
            MouseButton::Right => {
                self.gui.cursor.right_pressed = true;
                self.gui.fire_bubble_at(PressEvent, self.gui.cursor.position)?;
            }
        }
        Ok(())
    }

    pub fn release(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => {
                self.gui.cursor.left_pressed = false;
                let Some(pressed_id) = self.gui.pressed_id else { return Ok(()) };
                self.gui.pressed_id = None;
                let Some(node_touching_id) = self.gui.get_touching_id(self.gui.cursor.position) else { return Ok(()) };
                if node_touching_id != pressed_id { return Ok(()) }
                self.gui.fire_bubble(ReleaseEvent, pressed_id)?;
            },
            MouseButton::Right => {
                self.gui.cursor.right_pressed = false;
                self.gui.fire_global(ReleaseEvent)?;
            }
        }
        Ok(())
    }
}