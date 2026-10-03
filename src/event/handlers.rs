use std::{any::Any, marker::PhantomData};

use crate::{event::Event, world::World};

pub(crate) trait ErasedHandler {
    fn call(&mut self, world: &mut World, event: &dyn Any);
}

pub(crate) struct TypedHandler<E, F> {
    pub(crate) function: F,
    pub(crate) _event: PhantomData<E>,
}

impl<E, F> ErasedHandler for TypedHandler<E, F>
where
    E: Event,
    F: FnMut(&mut World, &E) + 'static,
{
    fn call(&mut self, world: &mut World, event: &dyn Any) {
        let event = event
            .downcast_ref::<E>()
            .expect("Event type was not expected for the handler!");

        (self.function)(world, event);
    }
}
