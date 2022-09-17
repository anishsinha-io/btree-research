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

use rayon::{ThreadPool, ThreadPoolBuilder};

const INDEX_PATH: &'static str = "/Users/anishsinha/Home/cprojects/playground/node/data/index.bin";

type Page = [u8; 512];


fn main() {
    prelude::cleanup();
    prelude::fmt_index();
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    pool.spawn(move || insert::ly_insert(1, 100));
    pool.spawn(move || insert::ly_insert(2, 200));
    pool.spawn(move || insert::ly_insert(3, 300));
    pool.spawn(move || insert::ly_insert(4, 400));
    pool.spawn(move || insert::ly_insert(5, 500));
    loop {}
}