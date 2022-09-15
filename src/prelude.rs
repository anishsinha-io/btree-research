use crate::{header::IndexHeader, node::{Node, ItemPtr}, fs, INDEX_PATH};

// setup index
pub fn fmt_index() {
    fs::truncate(INDEX_PATH);
    let mut header = IndexHeader::new(2);
    header.root_loc = ItemPtr { page_no: 0 };
    header.height = 0;
    let root = Node::new(2, ItemPtr { page_no: 1 }, ItemPtr { page_no: -1 }, vec![], None);
    header.write();
    root.write();
    println!("Formatted index");
}

pub fn cleanup() {
    fs::truncate(INDEX_PATH);
    println!("Cleared index");
}
