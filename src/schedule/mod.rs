//! Schedules are the object which runs systems on a world.

use rayon::{
    ThreadPool, ThreadPoolBuilder,
    iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator},
};

use crate::{
    system::{
        IntoSystem, System,
        access::{Access, AccessError},
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

/// Objects used to organize and run [`System`]'s
pub struct Schedule {
    systems: Vec<Box<dyn System>>,
    stages: Vec<Vec<usize>>,
    accesses: Vec<Access>,
    initialized: bool,
    pool: ThreadPool,
}

impl Default for Schedule {
    fn default() -> Self {
        let pool = ThreadPoolBuilder::new()
            .build()
            .expect("Failed to create worker pool");

        Self {
            systems: Vec::default(),
            stages: Vec::default(),
            accesses: Vec::default(),
            initialized: false,
            pool,
        }
    }
}

impl Schedule {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_system<S, M>(&mut self, sys: S) -> &mut Self
    where
        S: IntoSystem<M>,
    {
        self.systems.push(Box::new(sys.into_system()));

        self.stages.clear();
        self.accesses.clear();
        self.initialized = false;

        self
    }

    #[deprecated(since = "0.1.4", note = "Use the preferred 'add_system' method")]
    pub fn insert_system<S, M>(&mut self, sys: S) -> &mut Self
    where
        S: IntoSystem<M>,
    {
        self.add_system(sys)
    }

    pub fn initialize(&mut self, world: &mut World) -> Result<(), AccessError> {
        let mut accesses = Vec::with_capacity(self.systems.len());

        for system in &mut self.systems {
            system.initialize(world);

            let mut access = Access::default();
            system.access(&mut access)?;
            accesses.push(access);
        }

        let stages = {
            let mut stages: Vec<Vec<usize>> = Vec::new();

            for (id, access) in accesses.iter().enumerate() {
                let mut stage_idx = 0;

                loop {
                    if stage_idx == stages.len() {
                        stages.push(Vec::new());
                    }

                    let mut combined_access = access.clone();

                    let conflicts = stages[stage_idx]
                        .iter()
                        .any(|&other| self.systems[other].access(&mut combined_access).is_err());

                    if conflicts {
                        stage_idx += 1;
                    } else {
                        stages[stage_idx].push(id);
                        break;
                    }
                }
            }

            stages
        };

        self.accesses = accesses;
        self.stages = stages;
        self.initialized = true;

        Ok(())
    }

    pub fn run(&mut self, world: &mut World) {
        if !self.initialized {
            self.initialize(world).unwrap();
        }

        for stage in &self.stages {
            let (main_thread, parallel): (Vec<_>, Vec<_>) = stage
                .iter()
                .copied()
                .partition(|&index| self.systems[index].meta().is_main_thread_only());

            let world_cell = unsafe { UnsafeWorldCell::from_world(world) };

            for &index in &main_thread {
                unsafe {
                    self.systems[index].unsafe_run(world_cell);
                }
            }

            self.pool.install(|| {
                self.systems
                    .par_iter_mut()
                    .enumerate()
                    .filter(|(index, _)| parallel.contains(index))
                    .for_each(|(_, system)| unsafe {
                        system.unsafe_run(world_cell);
                    });
            });

            for &index in stage {
                self.systems[index].apply(world);
            }
        }
    }

    #[deprecated(since = "0.1.4", note = "Schedule initializes on the first run")]
    pub fn init_and_run(&mut self, world: &mut World) {
        self.initialize(world).unwrap();
        self.run(world);
    }
}
