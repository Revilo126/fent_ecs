//! Events can respond to certain triggers from systems.

use crate::event::handlers::ErasedHandler;

pub(crate) mod function_handlers;
pub(crate) mod handlers;
pub(crate) mod identification;

pub trait Event: Send + Sync + Sized + 'static {}

#[derive(Default)]
pub struct Events {
    pub(crate) handlers: Vec<Vec<Box<dyn ErasedHandler>>>,
}

impl Events {
    pub(crate) fn register(&mut self, id: usize, handler: Box<dyn ErasedHandler>) {
        if self.handlers.len() <= id {
            self.handlers.resize_with(id + 1, Vec::new);
        }
        self.handlers[id].push(handler);
    }

    pub(crate) fn take(&mut self, id: usize) -> Vec<Box<dyn ErasedHandler>> {
        self.handlers
            .get_mut(id)
            .map(std::mem::take)
            .unwrap_or_default()
    }

    pub(crate) fn restore(&mut self, id: usize, mut handlers: Vec<Box<dyn ErasedHandler>>) {
        if self.handlers.len() <= id {
            self.handlers.resize_with(id + 1, Vec::new);
        }
        handlers.append(&mut self.handlers[id]);
        self.handlers[id] = handlers;
    }
}
