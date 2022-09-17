use std::sync::{Arc, RwLock};
use crate::{Page, node::{Node, ItemPtr}, buffer, buffer::{GLOBAL_BUFFER_POOL, BufferPoolFrame}, bytes, fs, header::IndexHeader, table, table::GLOBAL_PAGE_TABLE};


pub fn read_header() -> Option<BufferPoolFrame> {
    if let Some(header) = buffer::fetch_frame(0) {
        println!("FOUND EXISTING HEADER PAGE IN BUFFER POOL");
        return Some(header);
    } else {
        println!("DID NOT FIND EXISTING HEADER IN BUFFER POOL");
        if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
            if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
                if let Some(header) = pages.get(0) {
                    if let Ok(locked) = header.read() {
                        return Some(locked.clone());
                    }
                } else {
                    let from_disk = IndexHeader::read();
                    let index = pages.len();
                    pages.insert(index, Arc::new(RwLock::new(BufferPoolFrame::new(0, from_disk))));
                    table.insert(0, index);
                    // assert because everything below should be guaranteed safe and unwrappable
                    return Some(pages.get(0).unwrap().read().unwrap().clone());
                }
            }
        }
    }
    None
}

pub fn ly_insert(val: usize, page_no: usize) {
    // initialize stack
    println!("{:#?}", std::thread::current().id());
    let mut stack: Vec<usize> = vec![];
    if let Some(header) = read_header() {
        println!("Thread: {:#?}, {}", std::thread::current().id(), IndexHeader::from_page(header.page_held));
    }
}

pub fn ly_insert_rec() {}