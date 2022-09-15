use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use lazy_static::lazy_static;

type PageTable = Arc<RwLock<HashMap<usize, usize>>>;

lazy_static! {
    pub static ref GLOBAL_PAGE_TABLE: PageTable = Arc::new(RwLock::new(HashMap::new()));
}

// get a key from GLOBAL_PAGE_TABLE in a thread-safe manner
pub fn get(key: usize) -> Option<usize> {
    if let Ok(table) = GLOBAL_PAGE_TABLE.read() {
        if let Some(val) = table.get(&key).clone() {
            return Some(*val);
        }
    }
    None
}

// insert a key-value pair into GLOBAL_PAGE_TABLE in a thread-safe manner
pub fn insert(key: usize, val: usize) {
    if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
        table.insert(key, val);
    }
}

// remove a key from GLOBAL_PAGE_TABLE in a thread-safe manner
pub fn remove(key: usize) {
    if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
        table.remove(&key);
    }
}

// get the length of the table in a thread-safe manner
pub fn len() -> Option<usize> {
    if let Ok(table) = GLOBAL_PAGE_TABLE.read() {
        return Some(table.len());
    }
    None
}