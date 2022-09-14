#![allow(dead_code, unused_variables)]

mod node;
mod bytes;
mod fs;

use std::io::{Result, Write};
use std::fs::File;
use node::NodePtr;
use node::Node;


const INDEX_PATH: &'static str = "/Users/anishsinha/Home/cprojects/playground/node/data/index.bin";

type Page = [u8; 512];

fn main() {
    let first_keys: Vec<usize> = vec![3];
    let first_children: Vec<NodePtr> = vec![NodePtr { page_no: 0 }, NodePtr { page_no: 1 }];
    let first = Node::new(2, NodePtr { page_no: 2 }, NodePtr { page_no: -1 }, first_keys, Some(first_children));
    println!("{}", first);
    let second_keys: Vec<usize> = vec![1, 2, 3];
    let second = Node::new(2, NodePtr { page_no: 0 }, NodePtr { page_no: 1 }, second_keys, None);
    let third_keys: Vec<usize> = vec![4, 5];
    let third = Node::new(2, NodePtr { page_no: 1 }, NodePtr { page_no: -1 }, third_keys, None);
    first.write();
    let first_page = first.read();
    println!("{}", Node::from_page(first_page));
}
