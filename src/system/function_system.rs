use crate::{
    system::{IntoSystem, System, access::Access},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub struct FunctionSystem<F> {
    function: F,
    access: Access,
}

impl<F> FunctionSystem<F> {
    pub fn new(function: F) -> Self {
        Self {
            function,
            access: Access::default(),
        }
    }
}

impl<F> System for FunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    fn run(&mut self, world: &mut World) {
        (self.function)(world);
    }

    fn access(&self) -> Access {
        self.access.clone()
    }

    unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>) {
        self.run(unsafe { world.world_mut() });
    }
}

pub struct FunctionSystemMarker;

impl<F> IntoSystem<FunctionSystemMarker> for F
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    type System = FunctionSystem<F>;

    fn into_system(self) -> Self::System {
        FunctionSystem::new(self)
    }
}
