use crate::{header::IndexHeader, node::{Node, NodePtr}, fs, INDEX_PATH};

// setup index
pub fn fmt_index() {
    fs::truncate(INDEX_PATH);
    let header = IndexHeader::new(2);
    let root = Node::new(2, NodePtr { page_no: 1 }, NodePtr { page_no: -1 }, vec![], None);
    header.write();
    root.write();
    println!("Formatted index");
}

pub fn cleanup() {
    fs::truncate(INDEX_PATH);
    println!("Cleared index");
}
