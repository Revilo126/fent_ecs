use std::{
    any::TypeId,
    collections::HashMap,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

pub type EventId = usize;

static EVENT_IDS: OnceLock<Mutex<HashMap<TypeId, EventId>>> = OnceLock::new();
static NEXT_EVENT_ID: AtomicUsize = AtomicUsize::new(0);

pub fn event_id<T: 'static>() -> EventId {
    let ids = EVENT_IDS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut ids = ids.lock().unwrap();

    let type_id = TypeId::of::<T>();

    if let Some(&id) = ids.get(&type_id) {
        return id;
    }

    let id = NEXT_EVENT_ID.fetch_add(1, Ordering::Relaxed);
    ids.insert(type_id, id);

    id
}
