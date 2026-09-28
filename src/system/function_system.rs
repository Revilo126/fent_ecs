use std::marker::PhantomData;

use crate::{
    system::{
        IntoSystem, System,
        access::{Access, AccessError},
        system_param::{ParamItem, SystemParam},
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub trait SystemParamFunction<Marker>: Send + Sync + 'static {
    type Param: SystemParam;
    fn run(&mut self, param: ParamItem<'_, Self::Param>);
}

pub struct FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    function: F,
    state: Option<<F::Param as SystemParam>::State>,
    marker: PhantomData<fn() -> Marker>,
}

impl<F, Marker: 'static> System for FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    fn initialize(&mut self, world: &mut World) {
        self.state = Some(F::Param::init_state(world));
    }

    fn access(&self, access: &mut Access) -> Result<(), AccessError> {
        let state = self.state.as_ref().expect("System was not initialized!");
        F::Param::access(state, access)
    }

    unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>) {
        let state = self.state.as_mut().expect("system not initialized");
        let param = unsafe { F::Param::get_param(state, world) };
        self.function.run(param);
    }

    fn run(&mut self, world: &mut World) {
        unsafe { self.unsafe_run(UnsafeWorldCell::from_world(world)) }
    }
}

impl<F, Marker> IntoSystem<Marker> for F
where
    F: SystemParamFunction<Marker>,
    Marker: 'static,
{
    type System = FunctionSystem<F, Marker>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            function: self,
            state: None,
            marker: PhantomData,
        }
    }
}

macro_rules! impl_system_param_tuple {
    ($($P:ident),*) => {
        #[allow(non_snake_case, unused_variables, unused_unsafe, clippy::unused_unit)]
        impl<$($P: SystemParam),*> SystemParam for ($($P,)*) {
            type State = ($($P::State,)*);
            type Item<'w> = ($($P::Item<'w>,)*);

            fn init_state(world: &mut World) -> Self::State {
                ($($P::init_state(world),)*)
            }

            fn access(state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
                let ($($P,)*) = state;
                $($P::access($P, access)?;)*
                Ok(())
            }

            unsafe fn get_param<'w>(
                state: &'w mut Self::State,
                world: UnsafeWorldCell<'w>,
            ) -> Self::Item<'w> {
                let ($($P,)*) = state;
                // SAFETY: caller guarantees access was validated.
                unsafe { ($($P::get_param($P, world),)*) }
            }
        }
    };
}

macro_rules! impl_system_param_function {
    ($($P:ident),*) => {
        #[allow(non_snake_case, unused_variables)]
        impl<Func, $($P: SystemParam),*> SystemParamFunction<fn($($P,)*)> for Func
        where
            Func: Send + Sync + 'static,
            for<'a> &'a mut Func:
                FnMut($($P),*) + FnMut($(ParamItem<'_, $P>),*),
        {
            type Param = ($($P,)*);

            fn run(&mut self, param: ParamItem<'_, ($($P,)*)>) {
                fn call_inner<$($P),*>(mut f: impl FnMut($($P),*), $($P: $P),*) {
                    f($($P),*)
                }
                let ($($P,)*) = param;
                call_inner(self, $($P),*)
            }
        }
    };
}

impl_system_param_tuple!();
impl_system_param_tuple!(P0);
impl_system_param_tuple!(P0, P1);
impl_system_param_tuple!(P0, P1, P2);
impl_system_param_tuple!(P0, P1, P2, P3);
impl_system_param_tuple!(P0, P1, P2, P3, P4);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5);

impl_system_param_function!();
impl_system_param_function!(P0);
impl_system_param_function!(P0, P1);
impl_system_param_function!(P0, P1, P2);
impl_system_param_function!(P0, P1, P2, P3);
impl_system_param_function!(P0, P1, P2, P3, P4);
impl_system_param_function!(P0, P1, P2, P3, P4, P5);
