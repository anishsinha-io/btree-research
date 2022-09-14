#![allow(dead_code, unused_variables)]

mod node;
mod bytes;
mod fs;
mod buffer;
mod table;

use std::io::{Result, Write};
use std::fs::File;
use node::NodePtr;
use node::Node;
use buffer::GLOBAL_BUFFER_POOL;


const INDEX_PATH: &'static str = "/Users/anishsinha/Home/cprojects/playground/node/data/index.bin";

type Page = [u8; 512];

fn setup() {
    fs::truncate(INDEX_PATH);
    let first_keys: Vec<usize> = vec![3];
    let first_children: Vec<NodePtr> = vec![NodePtr { page_no: 0 }, NodePtr { page_no: 1 }];
    let first = Node::new(2, NodePtr { page_no: 2 }, NodePtr { page_no: -1 }, first_keys, Some(first_children));
    let second_keys: Vec<usize> = vec![1, 2, 3];
    let second = Node::new(2, NodePtr { page_no: 0 }, NodePtr { page_no: 1 }, second_keys, None);
    let third_keys: Vec<usize> = vec![4, 5];
    let third = Node::new(2, NodePtr { page_no: 1 }, NodePtr { page_no: -1 }, third_keys, None);
    first.write();
    second.write();
    third.write();
}

fn main() {
    setup();
    let page1 = Node::read_loc(0);
    let page2 = Node::read_loc(1);
    let page3 = Node::read_loc(2);
    println!("{}", buffer::size().unwrap());
}
