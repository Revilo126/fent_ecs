//! Allows for chained systems that must run one after another.

use crate::{
    system::{
        IntoSystem, System,
        access::{Access, AccessError},
        meta::{SystemMeta, ThreadAccess::AnyThread},
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub trait SystemTuple: Send + Sync + 'static {
    fn initialize(&mut self, world: &mut World);

    fn run(&mut self, world: &mut World);

    fn access(&self, access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    ///
    /// The caller must ensure that all accesses requested by this system and it's subsidiaries are
    /// compatible with the accesses already active in the world. In particular,
    /// mutable accesses must not alias any other read or write access.
    unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>);

    fn apply(&mut self, world: &mut World);
}

macro_rules! impl_system_tuple {
    ($($SystemType:ident : $system:ident),+) => {
        impl<$($SystemType: System),+> SystemTuple for ($($SystemType,)+) {
            fn initialize(&mut self, world: &mut World) {
                let ($($system,)+) = self;
                $($system.initialize(world);)+
            }

            fn run(&mut self, world: &mut World) {
                let ($($system,)+) = self;
                $($system.run(world);)+
            }

            fn access(&self, _access: &mut Access) -> Result<(), AccessError> {
                let ($($system,)+) = self;

                $(
                    let mut access = Access::default();
                    $system.access(&mut access)?;
                )+

                Ok(())
            }

            unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>) {
                let ($($system,)+) = self;

                $(
                    // SAFETY: each child runs to completion before the next.
                    unsafe { $system.unsafe_run(world); }
                )+
            }

            fn apply(&mut self, world: &mut World) {
                let ($($system,)+) = self;
                $($system.apply(world);)+
            }
        }
    };
}

impl_system_tuple!(A: a);
impl_system_tuple!(A: a, B: b);
impl_system_tuple!(A: a, B: b, C: c);
impl_system_tuple!(A: a, B: b, C: c, D: d);
impl_system_tuple!(A: a, B: b, C: c, D: d, E: e);
impl_system_tuple!(A: a, B: b, C: c, D: d, E: e, F: f);

pub trait IntoSystemTuple<Markers> {
    type Systems: SystemTuple;

    fn into_system_tuple(self) -> Self::Systems;

    fn chain(self) -> ChainedSystem<Self::Systems>
    where
        Self: Sized,
    {
        ChainedSystem {
            systems: self.into_system_tuple(),
            meta: SystemMeta {
                name: "ChainedSystem",
                thread_access: AnyThread,
            },
        }
    }
}

macro_rules! impl_into_system_tuple {
    ($($Input:ident : $input:ident : $Marker:ident : $System:ident),+) => {
        impl<$($Input, $Marker, $System),+> IntoSystemTuple<($($Marker,)+)>
            for ($($Input,)+)
        where
            $($Input: IntoSystem<$Marker, System = $System>,)+
            $($System: System,)+
        {
            type Systems = ($($System,)+);

            fn into_system_tuple(self) -> Self::Systems {
                let ($($input,)+) = self;
                ($($input.into_system(),)+)
            }
        }
    };
}

impl_into_system_tuple!(A: a: MA: SA);
impl_into_system_tuple!(A: a: MA: SA, B: b: MB: SB);
impl_into_system_tuple!(A: a: MA: SA, B: b: MB: SB, C: c: MC: SC);
impl_into_system_tuple!(A: a: MA: SA, B: b: MB: SB, C: c: MC: SC, D: d: MD: SD);

// impl<S: SystemTuple> IntoSystem for S {
//    type System = ChainedSystem<S>;
//
//    fn into_system(self) -> Self::System {
//        ChainedSystem { systems: self }
//    }
//}

pub struct ChainedSystem<S> {
    systems: S,
    meta: SystemMeta,
}

impl<S: SystemTuple> System for ChainedSystem<S> {
    fn initialize(&mut self, world: &mut World) {
        self.systems.initialize(world);
    }

    fn run(&mut self, world: &mut World) {
        self.systems.run(world);
    }

    fn access(&self, access: &mut Access) -> Result<(), AccessError> {
        self.systems.access(access)
    }

    unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>) {
        // SAFETY: the children run sequentially. The scheduler must ensure
        // the chain's combined access is compatible with other active systems.
        unsafe {
            self.systems.unsafe_run(world);
        }
    }

    fn apply(&mut self, world: &mut World) {
        self.systems.apply(world);
    }

    fn meta(&self) -> &SystemMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut SystemMeta {
        &mut self.meta
    }
}

impl<S: SystemTuple> IntoSystem for ChainedSystem<S> {
    type System = Self;

    fn into_system(self) -> Self::System {
        self
    }
}
