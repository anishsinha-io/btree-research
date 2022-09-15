use std::sync::{Arc, RwLock};
use crate::{Page, node::{Node, ItemPtr}, buffer, buffer::GLOBAL_BUFFER_POOL, bytes, fs, header::IndexHeader, table, table::GLOBAL_PAGE_TABLE};

fn read_header() -> Option<Page> {
    if let Some(header) = buffer::read_page(0) {
        println!("FOUND EXISTING HEADER IN BUFFER POOL");
        return Some(header);
    } else {
        println!("DID NOT FIND EXISTING HEADER IN BUFFER POOL. LOCKING AND CHECKING AGAIN.");
        // acquire exclusive lock on global buffer pool
        if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
            // acquire exclusive lock on global page table
            if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
                // try to get the header
                if let Some(header) = pages.get(0) {
                    // if the header is there, acquire a shared lock on it
                    if let Ok(locked) = header.read() {
                        // return a clone of the header
                        return Some(locked.clone());
                    }
                } else {
                    // read the header from the disk
                    let from_disk = IndexHeader::read();
                    // insert it into the global buffer pool
                    let index = pages.len();
                    pages.insert(index, Arc::new(RwLock::new(from_disk)));
                    // insert it into the global page table
                    table.insert(0, index);
                    return Some(pages.get(0).unwrap().read().unwrap().clone());
                }
            }
        }
    }
    None
}

pub fn insert(val: usize, page_no: usize) {
    // initialize stack
    println!("{:#?}", std::thread::current().id());
    let mut stack: Vec<usize> = vec![];
    if let Some(header) = read_header() {
        println!("Thread: {:#?}, {}", std::thread::current().id(), IndexHeader::from_page(header));
    }
}

pub fn insert_recursive() {}