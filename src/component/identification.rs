//! Contains all objects and functions needed to indentify Components and Resources.

use std::{
    any::TypeId,
    collections::HashMap,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

// Components

/// [`ComponentId`]'s are simply a type for [`usize`]
pub type ComponentId = usize;

/// [`ComponentId`]'s are stored as a static [`HashMap`].
static COMPONENT_IDS: OnceLock<Mutex<HashMap<TypeId, ComponentId>>> = OnceLock::new();
/// To know what the next [`ComponentId`] is avaliable, a [`AtomicUsize`] is used to keep track.
static NEXT_COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);

pub fn component_id<T: 'static>() -> ComponentId {
    let ids = COMPONENT_IDS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut ids = ids.lock().unwrap();

    let type_id = TypeId::of::<T>();

    if let Some(&id) = ids.get(&type_id) {
        return id;
    }

    let id = NEXT_COMPONENT_ID.fetch_add(1, Ordering::Relaxed);
    ids.insert(type_id, id);

    id
}

// Resources

/// [`ResourceId`]'s are simply a type for [`usize`]
pub type ResourceId = usize;

/// [`ResourceId`]'s are stored as a static [`HashMap`]
static RESOURCE_IDS: OnceLock<Mutex<HashMap<TypeId, usize>>> = OnceLock::new();
/// To know what the next [`ResourceId`] is avaliable, a [`AtomicUsize`] is used to keep track
static NEXT_RESOURCE_ID: AtomicUsize = AtomicUsize::new(0);

pub fn resource_id<T: 'static>() -> usize {
    let ids = RESOURCE_IDS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut ids = ids.lock().unwrap();

    let type_id = TypeId::of::<T>();

    if let Some(&id) = ids.get(&type_id) {
        return id;
    }

    let id = NEXT_RESOURCE_ID.fetch_add(1, Ordering::Relaxed);
    ids.insert(type_id, id);

    id
}
