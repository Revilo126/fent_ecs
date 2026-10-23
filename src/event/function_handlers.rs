use std::marker::PhantomData;

use crate::{
    event::{
        Event,
        handlers::{EventHandler, IntoEventHandler},
    },
    system::{
        access::{Access, AccessError},
        system_param::{ParamItem, SystemParam},
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub trait EventHandlerFunction<E: Event, Marker>: Send + Sync + 'static {
    type Param: SystemParam;
    fn run(&mut self, event: &E, param: ParamItem<'_, Self::Param>);
}

impl<E, F, Marker> IntoEventHandler<E, Marker> for F
where
    E: Event,
    F: EventHandlerFunction<E, Marker>,
    Marker: 'static,
{
    type Handler = FunctionEventHandler<E, F, Marker>;

    fn into_event_handler(self) -> Self::Handler {
        FunctionEventHandler {
            function: self,
            state: None,
            marker: PhantomData,
        }
    }
}

pub struct FunctionEventHandler<E, F, Marker>
where
    E: Event,
    F: EventHandlerFunction<E, Marker>,
{
    function: F,
    state: Option<<F::Param as SystemParam>::State>,
    marker: PhantomData<fn(&E) -> Marker>,
}

impl<E, F, Marker> EventHandler<E> for FunctionEventHandler<E, F, Marker>
where
    E: Event,
    F: EventHandlerFunction<E, Marker>,
    Marker: 'static,
{
    fn initialize(&mut self, world: &mut World) {
        self.state = Some(F::Param::init_state(world));
    }

    fn access(&self, access: &mut Access) -> Result<(), AccessError> {
        let state = self
            .state
            .as_ref()
            .expect("Event handler was not initialized!");
        F::Param::access(state, access)
    }

    fn run(&mut self, event: &E, world: &mut World) {
        // Same validated path as a system.
        unsafe { self.unsafe_run(event, UnsafeWorldCell::from_world(world)) }
    }

    unsafe fn unsafe_run(&mut self, event: &E, world: UnsafeWorldCell<'_>) {
        let state = self
            .state
            .as_mut()
            .expect("Event handler was not initialized!");
        let param = unsafe { F::Param::get_param(state, world) };
        self.function.run(event, param);
    }

    fn apply(&mut self, world: &mut World) {
        F::Param::apply(
            self.state
                .as_mut()
                .expect("Event handler was not initialized!"),
            world,
        );
    }
}

macro_rules! impl_event_handler_function {
    ($($P:ident),*) => {
        #[allow(non_snake_case, unused_variables, clippy::unused_unit)]
        impl<E, Func, $($P: SystemParam),*>
            EventHandlerFunction<E, fn(&E $(, $P)*)> for Func
        where
            E: Event,
            Func: Send + Sync + 'static,
            for<'e, 'w> Func: FnMut(
                &'e E
                $(, ParamItem<'w, $P>)*
            ),
        {
            type Param = ($($P,)*);

            fn run<'e, 'w>(
                &mut self,
                event: &'e E,
                param: ParamItem<'w, Self::Param>,
            ) {
                let ($($P,)*) = param;
                self(event $(, $P)*);
            }
        }
    };
}

impl_event_handler_function!();
impl_event_handler_function!(P0);
impl_event_handler_function!(P0, P1);
impl_event_handler_function!(P0, P1, P2);
impl_event_handler_function!(P0, P1, P2, P3);
impl_event_handler_function!(P0, P1, P2, P3, P4);
impl_event_handler_function!(P0, P1, P2, P3, P4, P5);
