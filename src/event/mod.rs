//! Events can respond to certain triggers from systems.

use std::{any::Any, marker::PhantomData};

use crate::{
    event::{
        handlers::{ErasedHandler, TypedHandler},
        identification::event_id,
    },
    world::World,
};

pub(crate) mod handlers;
pub(crate) mod identification;

pub trait Event: Send + Sync + Sized + 'static {}

#[derive(Default)]
pub struct Events {
    pub(crate) handlers: Vec<Vec<Box<dyn ErasedHandler>>>,
}

impl Events {
    pub fn on<E, F>(&mut self, handler: F)
    where
        E: Event,
        F: FnMut(&mut World, &E) + 'static,
    {
        let id = event_id::<E>();

        let handler = TypedHandler {
            function: handler,
            _event: PhantomData::<E>,
        };

        if self.handlers.len() <= id {
            self.handlers.resize_with(id + 1, Vec::new);
        }

        self.handlers[id].push(Box::new(handler));
    }

    pub fn emit<E: Event>(&mut self, world: &mut World, event: E) {
        let id = event_id::<E>();

        if id >= self.handlers.len() {
            return;
        }

        let event = &event as &dyn Any;

        let mut handlers = std::mem::take(&mut self.handlers[id]);

        for handler in &mut handlers {
            handler.call(world, event);
        }

        self.handlers[id] = handlers;
    }
}
