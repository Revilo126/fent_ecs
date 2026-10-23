use std::any::Any;

use crate::{
    event::Event,
    system::access::{Access, AccessError},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

// Event handlers are almost identicle to Systems, with the difference that they aren't run by
// schedules but instead when an event is called.

pub trait EventHandler<E: Event>: Send + Sync + 'static {
    fn initialize(&mut self, world: &mut World);

    fn run(&mut self, event: &E, world: &mut World);

    fn access(&self, access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    ///
    /// The caller must ensure that all accesses requested by this event handler are
    /// compatible with the accesses already active in the world. In particular,
    /// mutable accesses must not alias any other read or write access.
    unsafe fn unsafe_run(&mut self, event: &E, world: UnsafeWorldCell<'_>);

    /// Runs after the event handler is finished, useful for applying actions after completion.
    fn apply(&mut self, world: &mut World);
}

pub trait IntoEventHandler<E: Event, Marker = ()> {
    type Handler: EventHandler<E>;

    fn into_event_handler(self) -> Self::Handler;
}

pub trait ErasedHandler {
    fn initialize(&mut self, world: &mut World);
    fn access(&self, access: &mut Access) -> Result<(), AccessError>;
    fn call(&mut self, world: &mut World, event: &dyn Any);
    fn apply(&mut self, world: &mut World);
}

pub struct TypedHandler<E: Event> {
    pub(crate) handler: Box<dyn EventHandler<E>>,
}

impl<E: Event> ErasedHandler for TypedHandler<E> {
    fn initialize(&mut self, world: &mut World) {
        self.handler.initialize(world);
    }

    fn access(&self, access: &mut Access) -> Result<(), AccessError> {
        self.handler.access(access)
    }

    fn call(&mut self, world: &mut World, event: &dyn Any) {
        let event = event
            .downcast_ref::<E>()
            .expect("Event type did not match its handler");
        self.handler.run(event, world);
    }

    fn apply(&mut self, world: &mut World) {
        self.handler.apply(world);
    }
}
