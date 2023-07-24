
use std::any::Any;

use crate::{NodeId, Name};

/// Any event type.
pub trait Event: Any + 'static {}

/// Helpful dynamic wrapper around an [`Event`].
pub struct DynEvent(Box<dyn Any>);
impl DynEvent {
    pub(crate) fn new(event: impl Event) -> Self {
        Self(Box::new(event))
    }
    pub fn as_event<E: Event>(&self) -> Option<&E> {
        self.0.downcast_ref()
    }
    pub fn is_event<E: Event>(&self) -> bool {
        self.0.downcast_ref::<E>().is_some()
    }
}

impl<E: Event> From<E> for DynEvent {
    fn from(e: E) -> Self {
        Self::new(e)
    }
}

/// Information about a node where an event originated.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct NodeOrigin {
    pub id: NodeId,
    pub name: Option<Name>
}

/// Represents an event "control flow".
/// Provides an event that occured, on what node it occurred (if any),
/// and allows for the user to stop the propogation of the event if they desire.
pub struct EventControl<'e> {
    /// Event that was fired.
    pub event: &'e DynEvent,
    /// Info about the descendant node that fired the event, if any.
    pub(crate) origin: Option<NodeOrigin>,
    /// Writable. Events to be fired after handling this event.
    pub(crate) outgoing_events: Vec<DynEvent>,
    /// Writable. If set to true, event will not be handled by ancestors.
    /// This has not affect on global events.
    pub(crate) stop: bool,
    pub(crate) repaint: bool,
    pub(crate) pressed: bool
}

impl<'e> EventControl<'e> {

    pub(crate) fn new(event: &'e DynEvent, origin: Option<NodeOrigin>) -> Self {
        Self {
            event,
            origin,
            outgoing_events: Vec::new(),
            stop: false,
            repaint: false,
            pressed: false
        }
    }

    /// Info about the descendant node that the event came from.
    pub fn origin(&self) -> Option<NodeOrigin> {
        self.origin
    }

    /// True if the event is the type specified.
    pub fn is_event<E: Event>(&self) -> bool {
        self.event.is_event::<E>()
    }

    /// Tries to downcast the event to the type specified.
    pub fn as_event<E: Event>(&self) -> Option<&E> {
        self.event.as_event::<E>()
    }

    /// Returns [`Option::Some`] if the event type matches and the name of the node it originated from matches.
    pub fn matches_event<E: Event>(&self, node_name: impl Into<Option<Name>>) -> Option<(&'e E, NodeOrigin)> {
        let name = node_name.into();
        let event = self.event.as_event::<E>()?;
        let origin = self.origin?;
        if let Some(name) = name {
            let origin_name = origin.name?;
            if origin_name != name { return None }
        }
        Some((event, origin))
    }

    /// Fires an outgoing event for ancestors to react to.
    pub fn fire(&mut self, event: impl Into<DynEvent>) {
        self.outgoing_events.push(event.into());
    }

    /// If invoked for a bubble [`Event`], will stop the event propagation to further ancestors.
    pub fn stop(&mut self) {
        self.stop = true;
    }

    /// If invoked, will trigger a repaint of the [`crate::Widget`] that called it.
    pub fn repaint(&mut self) {
        self.repaint = true;
    }

    /// Marks the [`crate::Node`] of the [`crate::Widget`] as "pressed".
    pub fn press(&mut self) {
        self.pressed = true;
    }
}

/// Allows the user to write outgoing events.
pub struct OutgoingEvents(pub(crate) Vec<DynEvent>);
impl OutgoingEvents {
    pub fn push(&mut self, event: impl Into<DynEvent>) {
        self.0.push(event.into())
    }
}


// Built-in events
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct PressEvent;
impl Event for PressEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct ReleaseEvent;
impl Event for ReleaseEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct GuiEnterEvent;
impl Event for GuiEnterEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct GuiExitEvent;
impl Event for GuiExitEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct EnterEvent;
impl Event for EnterEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct ExitEvent;
impl Event for ExitEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct FocusEvent;
impl Event for FocusEvent {}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct UnfocusEvent;
impl Event for UnfocusEvent {}