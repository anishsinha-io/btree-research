use std::sync::{Arc, RwLock};
use crate::{node::{Node, NodePtr}, buffer, buffer::GLOBAL_BUFFER_POOL, bytes, fs, header::IndexHeader, table, table::GLOBAL_PAGE_TABLE};

fn get_root_loc() -> Option<NodePtr> {
    // get the location of the header in the buffer pool from the page table
    if let Some(val) = table::get(0) {
        // get the location of the header
        if let Some(page) = buffer::read_page(val) {
            // read the header and return the root's location
            let header = IndexHeader::from_page(page);
            Some(header.root_loc)
        } else {
            // if we're here the header page was dropped in between getting its location and reading from the buffer
            // pool so just panic
            None
        }
    } else {
        // if we're here it means that we couldn't find the header in the buffer pool, so we need to read it from
        // disk
        use buffer::GLOBAL_BUFFER_POOL;
        // get an exclusive lock on the global buffer pool
        if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
            // check to make sure the header isn't here
            return if let Some(header_page) = pages.get(0) {
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
                Some(header.root_loc)
            };
        }
        None
    }
}

pub fn insert(val: usize, page_no: usize) {
    // initialize stack
    let mut stack: Vec<usize> = vec![];
    // get root's location
    let root_loc = get_root_loc().unwrap();
    // read the root
    let root = Node::read_loc(root_loc.page_no as u64);
}

pub fn insert_recursive() {}