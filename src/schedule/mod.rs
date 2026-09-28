//! Schedules are the object which runs systems on a world.

use crate::{
    system::{IntoSystem, System, access::Access},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

#[derive(Default)]
pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn insert_system<S>(&mut self, sys: S)
    where
        S: IntoSystem<()>,
    {
        self.systems.push(Box::new(sys.into_system()));
    }

    pub fn run(&mut self, world: &mut World) {
        let unsafe_world = unsafe { UnsafeWorldCell::from_world(world) };

        let mut access = Access::default();

        for sys in &mut self.systems {
            unsafe { sys.unsafe_run(unsafe_world, &mut access) };
        }
    }
}
