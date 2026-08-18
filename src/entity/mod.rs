//! Entities are the basis (of objects) in an ECS library.
//! In Fent ECS they are simply typed usize integers.

use std::collections::VecDeque;

// The Entity type, it is a usize so that on 32-bit systems it's a u32
// While on 64-bit it will be a u64 integer.
pub type Entity = usize;

/// The storage of [`Entity`]'s can be tricky if an entity is removed,
/// So the [`Entities`] struct is made to help!
#[derive(Default)]
pub struct Entities {
    /// [`VecDeque`] to keep track
    pub(crate) free: VecDeque<Entity>,
    /// Next avaliable [`Entity`]
    pub(crate) next: Entity,
    /// All [`Entity`]'s already in use
    pub(crate) in_use: Vec<bool>,
}

impl Entities {
    /// Allocates a new (or free) [`Entity`] within itself,
    /// assigned to the next number in the queue
    pub fn alloc(&mut self) -> Entity {
        if let Some(id) = self.free.pop_front() {
            self.in_use[id] = true;
            id
        } else {
            let id = self.next;
            self.next += 1;

            if id >= self.in_use.len() {
                self.in_use.resize(id + 1, false);
            }

            self.in_use[id] = true;
            id
        }
    }

    /// Free an [`Entity`]'s Id for use later
    pub fn free(&mut self, id: Entity) {
        if id >= self.in_use.len() || !self.in_use[id] {
            log::error!("Tried to free an out of bounds/already freed entity!");
            return;
        }

        self.in_use[id] = false;
        self.free.push_back(id);
    }

    /// Retrieve the amount of [`Entities`] allocated,
    ///
    /// Can possibly get laggy with large quantities.
    pub fn entity_allocated(&self) -> usize {
        self.in_use.iter().filter(|&&v| v).count()
    }
}
