use std::sync::RwLock;

pub static NUMBER_OF_WORKERS: RwLock<usize> = RwLock::new(1);
