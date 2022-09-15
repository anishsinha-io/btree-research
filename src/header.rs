use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::{bytes, fs, Page, INDEX_PATH, node::NodePtr};

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct IndexHeader {
    pub min_order: u32,
    pub root_loc: NodePtr,
    pub height: i32,
}

impl Display for IndexHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexHeader [min_order={} root_loc={} height={}]", self.min_order, self.root_loc, self.height)
    }
}

impl IndexHeader {
    pub fn new(min_order: u32) -> Self {
        IndexHeader {
            min_order,
            root_loc: NodePtr { page_no: -1 },
            height: -1,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        bytes::encode(self).unwrap()
    }

    pub fn decode(bytes: Vec<u8>) -> Self {
        bytes::decode(bytes).unwrap()
    }

    pub fn as_page(&self) -> [u8; 512] {
        let bytes = self.encode();
        let size = bytes.len();
        let mut buf = [0u8; 512];
        buf[..size].clone_from_slice(&bytes);
        buf
    }

    pub fn from_page(buf: Page) -> Self {
        Self::decode(buf.to_vec())
    }


    pub fn write(&self) {
        fs::write_bytes(INDEX_PATH, self.as_page(), 0);
    }

    pub fn read() -> Page {
        let mut buf = [0u8; 512];
        fs::read_bytes(INDEX_PATH, &mut buf, 0);
        buf
    }
}