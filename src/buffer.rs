use std::sync::{Arc, Mutex, RwLock};
use lazy_static::lazy_static;
use crate::Page;

type BufferPool = Arc<RwLock<Vec<Arc<RwLock<Page>>>>>;

lazy_static! {
   pub static ref GLOBAL_BUFFER_POOL: BufferPool = Arc::new(RwLock::new(Vec::new()));
}

pub fn add_page(new_page: Page) {
    //  acquire an exclusive lock on the pool
    if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
        // push the new page
        pages.push(Arc::new(RwLock::new(new_page)));
        // exclusive lock is released here
    }
}

pub fn read_page(loc: usize) -> Option<Page> {
    // acquire a shared lock on the pool
    if let Ok(pages) = GLOBAL_BUFFER_POOL.read() {
        // acquire a shared lock on the page (this is against Lehman and Yao's algorithm but fits with
        // Lanin and Shasha's more modern approach which guarantees correct behavior.
        if let Ok(page) = pages.get(loc).unwrap().read() {
            // return a clone of the page
            return Some((*page).clone());
            // shared lock on page is released here
        }
        // shared lock on pool is released here
    }
    None
}

pub fn update_page(loc: usize, updated_page: Page) {
    // acquire a shared lock on the pool
    if let Ok(pages) = GLOBAL_BUFFER_POOL.read() {
        // acquire an exclusive lock on the page
        if let Ok(mut page) = pages.get(loc).unwrap().write() {
            // update page
            *page = updated_page;
            // exclusive lock on page is released here
        }
        // exclusive lock on pool is released here
    }
}

pub fn drop_page(loc: usize) {
    // acquire an exclusive lock on the pool
    if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
        // remove page
        pages.remove(loc);
    }
}

pub fn size() -> Option<usize> {
    if let Ok(pages) = GLOBAL_BUFFER_POOL.read() {
        return Some(pages.len());
    }
    None
}