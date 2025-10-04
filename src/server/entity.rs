use std::sync::atomic::{AtomicI32, Ordering};

const ENTITY_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

fn next_entity_id() -> i32 {
    ENTITY_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug)]
pub struct Entity {
    entity_id: i32,
}

impl Entity {
    pub fn new() -> Self {
        Self { entity_id: next_entity_id() }
    }

    pub fn entity_id(&self) -> i32 {
        self.entity_id
    }
}
