use std::sync::{Arc, RwLock};
use crate::{Page, node::{Node, ItemPtr}, buffer, buffer::GLOBAL_BUFFER_POOL, bytes, fs, header::IndexHeader, table, table::GLOBAL_PAGE_TABLE};

fn get_root_loc() -> Option<ItemPtr> {
    // get the location of the header in the buffer pool from the page table
    if let Some(val) = table::get(0) {
        // get the location of the header
        if let Some(page) = buffer::read_page(val) {
            // read the header and return the root's location
            let header = IndexHeader::from_page(page);
            return Some(header.root_loc);
        } else {
            // if we're here the header page was dropped in between getting its location and reading from the buffer
            // pool so just panic
            None
        }
    } else {
        // if we're here it means that we couldn't find the header in the buffer pool, so we need to read it from
        // disk
        // get an exclusive lock on the global buffer pool
        if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
            // check to make sure the header isn't here
            if let Some(header_page) = pages.get(0) {
                // if it is, just return the root's location
                if let Ok(page) = header_page.read() {
                    let header = IndexHeader::from_page(*page);
                    Some(header.root_loc)
                } else {
                    // if the header was here and not in the page table, and we can't read it then something is seriously
                    // wrong, so panic
                    None
                }
            } else {
                // retaining our lock on the buffer pool, we can safely insert the new page, read from disk, into it at the
                // end
                let header_from_disk = IndexHeader::read();
                let index: usize = pages.len();
                pages.insert(index, Arc::new(RwLock::new(header_from_disk)));
                if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
                    // make absolutely sure that the only root location in the page table is ours
                    table.remove(&0usize);
                    table.insert(0, index);
                }
                let header = IndexHeader::from_page(header_from_disk);
                // return the root's location in the buffer pool
                return Some(header.root_loc);
            };
        }
        None
    }
}

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