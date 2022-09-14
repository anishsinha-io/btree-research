use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use lazy_static::lazy_static;

type PageTable = Arc<RwLock<HashMap<usize, usize>>>;

lazy_static! {
    pub static ref GLOBAL_PAGE_TABLE: PageTable = Arc::new(RwLock::new(HashMap::new()));
}

pub fn get(key: usize) -> Option<usize> {
    if let Ok(table) = GLOBAL_PAGE_TABLE.read() {
        return Some((*table.get(&key).clone()));
    }
    None
}

pub fn set(key: usize, val: usize) {
    if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
        table.insert(key, val);
    }
}

pub fn remove(key: usize) {
    if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
        table.remove(&key);
    }
}