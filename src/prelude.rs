//! Contains most objects for a normal program.
//!
//! They are still accessible through their respective 'use' statements.

pub use fent_derive::{Component, Event, Message, Resource};

pub use crate::{
    component::{
        identification::{component_id, resource_id},
        storage::{sparse_set::SparseStorage, vec::VecStorage},
    },
    entity::{Entities, Entity},
    schedule::Schedule,
    system::{
        IntoSystem, System,
        chain_system::IntoSystemTuple,
        impl_param::{
            Local, Res, ResMut,
            command::Commands,
            messages::{MessageReader, MessageWriter},
            non_send::NonSendMarker,
            queries::{Query, With, Without},
        },
        system_param::SystemParam,
    },
    world::World,
};
