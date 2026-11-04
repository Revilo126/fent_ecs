//! Messages module of fent_ecs

use crate::component::resource::Resource;

/// Marker trait for Messages
pub trait Message: Send + Sync + 'static {}

pub struct Messages<M: Message> {
    pub(crate) queue: Vec<M>,
}

impl<T: Message> Resource for Messages<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<M: Message> Default for Messages<M> {
    fn default() -> Self {
        Self { queue: Vec::new() }
    }
}

impl<M: Message> Messages<M> {
    pub(crate) fn write(&mut self, message: M) {
        self.queue.push(message);
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, M> {
        self.queue.iter()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.queue.clear();
    }
}
