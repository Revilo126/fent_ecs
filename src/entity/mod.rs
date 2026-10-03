//! Entities are the basis (of objects) in an ECS library.
//! In Fent ECS they are simply typed usize integers.

use std::collections::VecDeque;

/// The Entity type
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Entity {
    idx: usize,
    generation: usize,
}

impl Entity {
    #[inline]
    pub fn index(&self) -> usize {
        self.idx
    }

    #[inline]
    pub fn generation(&self) -> usize {
        self.generation
    }
}

/// The storage of [`Entity`]'s can be tricky if an entity is removed,
/// So the [`Entities`] struct is made to help!
#[derive(Default)]
pub struct Entities {
    /// [`VecDeque`] to keep track
    pub(crate) free: VecDeque<usize>,
    /// The generation of an Entity index
    pub(crate) generations: Vec<usize>,
    /// All [`Entity`]'s already in use
    pub(crate) in_use: Vec<bool>,
}

impl Entities {
    /// Spawns a [`Entity`] with a new (or free) id within itself,
    /// assigned to the next number in the queue
    pub fn spawn(&mut self) -> Entity {
        if let Some(idx) = self.free.pop_front() {
            self.in_use[idx] = true;
            Entity {
                idx,
                generation: self.generations[idx],
            }
        } else {
            let idx = self.generations.len();
            self.generations.push(0);
            self.in_use.push(true);
            Entity { idx, generation: 0 }
        }
    }

    /// Despawns an [`Entity`]'s Id for use later
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }
        let idx = entity.idx;
        self.in_use[idx] = false;
        self.generations[idx] = self.generations[idx].wrapping_add(1);
        self.free.push_back(entity.idx);
        true
    }

    /// Retrieve the amount of [`Entities`] allocated,
    ///
    /// Can possibly get laggy with large quantities.
    pub fn entity_allocated(&self) -> usize {
        self.in_use.iter().filter(|&&v| v).count()
    }

    /// Checks if an [`Entity`] is alive.
    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.index();
        self.in_use.get(idx).copied().unwrap_or(false)
            && self.generations.get(idx).copied() == Some(entity.generation)
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.in_use.iter().enumerate().filter_map(|(i, &used)| {
            used.then_some(Entity {
                idx: i,
                generation: self.generations[i],
            })
        })
    }
}
