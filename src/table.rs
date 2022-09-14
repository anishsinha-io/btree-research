use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use lazy_static::lazy_static;

type PageTable = Arc<RwLock<HashMap<usize, usize>>>;

lazy_static! {
    pub static ref GLOBAL_PAGE_TABLE: PageTable = Arc::new(RwLock::new(HashMap::new::<usize, usize>()))
}