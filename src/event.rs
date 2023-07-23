
use std::any::Any;

use crate::{NodeId, Name};

/// Any event type.
pub trait Event: Any + 'static {}

/// Helpful dynamic wrapper around an [`Event`].
pub struct DynEvent {
    event: Box<dyn Any>
}
impl DynEvent {
    pub(crate) fn new(event: impl Event) -> Self {
        Self {
            event: Box::new(event)
        }
    }
    pub fn into<E: Event>(&self) -> Option<&E> {
        self.event.downcast_ref()
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
pub struct EventControl<'e> {
/// and allows for the user to stop the propogation of the event if they desire.
    /// Event that was fired.
    pub event: &'e DynEvent,
    /// Info about the descendant node that fired the event.
    pub origin: Option<NodeOrigin>,
    /// Writable. Events to be fired after handling this event.
    pub outgoing_events: OutgoingEvents,
    /// Writable. If set to true, event will not be handled by ancestors.
    pub stop: bool,
    /// Writable. If set to true, will trigger a repaint of the widget handling the event.
    pub repaint: bool
}

impl<'e> EventControl<'e> {

    pub(crate) fn new(event: &'e DynEvent, origin: Option<NodeOrigin>) -> Self {
        Self {
            event,
            origin,
            outgoing_events: OutgoingEvents(Vec::new()),
            stop: false,
            repaint: false
        }
    }

    pub fn matches<E: Event>(&self) -> Option<&'e E> {
        self.event.into::<E>()
    }

    pub fn event<E: Event>(&self, name: impl Into<Option<Name>>) -> Option<(&'e E, NodeOrigin)> {
        let name = name.into();
        let event = self.event.into::<E>()?;
        let origin = self.origin?;
        let origin_name = origin.name?;
        if let Some(name) = name {
            if origin_name != name { return None }
        }
        Some((event, origin))
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