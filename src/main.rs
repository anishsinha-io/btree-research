#![allow(dead_code, unused_variables)]

mod node;
mod bytes;
mod fs;
mod buffer;
mod table;
mod header;
mod insert;
mod delete;
mod search;
mod prelude;

use std::fs::File;
use node::NodePtr;
use node::Node;
use buffer::GLOBAL_BUFFER_POOL;

const INDEX_PATH: &'static str = "/Users/anishsinha/Home/cprojects/playground/node/data/index.bin";

type Page = [u8; 512];

fn main() {
    prelude::fmt_index();
    prelude::cleanup();
}
