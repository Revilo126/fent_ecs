//! Entities are the basis (of objects) in an ECS library.
//!
//! In Fent ECS they are structs containing their index and generation.

use std::collections::VecDeque;

/// The Entity type
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Entity {
    idx: usize,
    generation: usize,
}

impl Entity {
    /// Returns the index of current [`Entity`]
    #[inline]
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Returns the generation of the [`Entity`]
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

    /// Returns the youngest (newest) generation alive
    ///
    /// If the [`Vec`] of generations or in_use is empty returns a [`None`]
    pub fn youngest_generation(&self) -> Option<usize> {
        self.generations
            .iter()
            .zip(&self.in_use)
            .filter_map(|(&generation, &used)| used.then_some(generation))
            .max()
    }

    /// Returns the oldest generation alive
    ///
    /// If the [`Vec`] of generations or in_use is empty returns a [`None`]
    pub fn oldest_generation(&self) -> Option<usize> {
        self.generations
            .iter()
            .zip(&self.in_use)
            .filter_map(|(&generation, &used)| used.then_some(generation))
            .min()
    }

    /// Returns an [`Iterator`] over all alive
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.in_use.iter().enumerate().filter_map(|(i, &used)| {
            used.then_some(Entity {
                idx: i,
                generation: self.generations[i],
            })
        })
    }

    /// Returns a copy of the free id [`VecDeque`]
    #[inline]
    pub fn free_ids(&self) -> VecDeque<usize> {
        self.free.clone()
    }

    /// Returns a copy of the generations [`Vec`]
    #[inline]
    pub fn generations(&self) -> Vec<usize> {
        self.generations.clone()
    }

    /// Returns a copy of the entities in use [`Vec`]
    #[inline]
    pub fn in_use(&self) -> Vec<bool> {
        self.in_use.clone()
    }
}
