//! Contains most objects for a normal program.
//!
//! They are still accessible through their respective 'use' statements.

pub use fent_derive::{Component, Event, Message, Resource};

pub use crate::{
    entity::Entity,
    schedule::Schedule,
    system::{
        IntoSystem, System,
        chain_system::IntoSystemTuple,
        impl_param::{
            Local, Res, ResMut,
            command::Commands,
            messages::{MessageReader, MessageWriter},
            queries::{Query, With, Without},
        },
        system_param::SystemParam,
    },
    world::World,
};
