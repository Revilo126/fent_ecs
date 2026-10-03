//! Schedules are the object which runs systems on a world.

use crate::{
    system::{
        IntoSystem, System,
        access::{Access, AccessError},
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

#[derive(Default)]
pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn insert_system<S, M>(&mut self, sys: S) -> &mut Self
    where
        S: IntoSystem<M>,
    {
        self.systems.push(Box::new(sys.into_system()));
        self
    }

    pub fn initialize(&mut self, world: &mut World) -> Result<(), AccessError> {
        for system in &mut self.systems {
            system.initialize(world);

            let mut access = Access::default();
            system.access(&mut access)?;
        }

        Ok(())
    }

    pub fn run(&mut self, world: &mut World) {
        for sys in &mut self.systems {
            let unsafe_world = unsafe { UnsafeWorldCell::from_world(world) };
            unsafe { sys.unsafe_run(unsafe_world) };

            sys.apply(world);
        }
    }
}
