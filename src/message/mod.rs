//! Messages module of fent_ecs

use fent_derive::Resource;

/// Marker trait for Messages
pub trait Message: Send + Sync + 'static {}

#[derive(Resource)]
pub struct Messages<M: Message> {
    pub(crate) queue: Vec<M>,
}

impl<M: Message> Default for Messages<M> {
    fn default() -> Self {
        Self { queue: Vec::new() }
    }
}

impl<M: Message> Messages<M> {
    #[inline]
    pub(crate) fn write(&mut self, message: M) {
        self.queue.push(message);
    }

    #[inline]
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, M> {
        self.queue.iter()
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.queue.clear();
    }
}
