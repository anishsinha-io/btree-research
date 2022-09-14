use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

use crate::{bytes, Page};
use crate::fs;
use crate::INDEX_PATH;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodePtr {
    pub page_no: i32,
}

impl Display for NodePtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemPtr [page_no={}]", self.page_no)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub min_order: u32,
    pub num_keys: u32,
    pub leaf: bool,
    pub loc: NodePtr,
    pub link: NodePtr,
    pub keys: Vec<usize>,
    pub high_key: usize,
    pub children: Vec<NodePtr>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node [order={} num_keys={} leaf={} loc={} link={} keys={:#?} high_key={} children={:#?}]",
               self.min_order, self.num_keys, self.leaf, self.loc, self.link, self.keys, self.high_key, self.children)
    }
}

impl Node {
    pub fn new(order: u32, loc: NodePtr, link: NodePtr, keys: Vec<usize>, children: Option<Vec<NodePtr>>) -> Self {
        let high_key = *keys.iter().max().unwrap();
        Node {
            min_order: order,
            loc,
            link,
            num_keys: 0,
            leaf: true,
            keys,
            high_key,
            children: if children != None { children.unwrap() } else { vec![] },
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
        let offset: u64 = 512 * self.loc.page_no as u64;
        fs::write_bytes(INDEX_PATH, self.as_page(), offset);
    }

    pub fn read(&self) -> Page {
        let offset: u64 = 512 * self.loc.page_no as u64;
        let mut buf = [0u8; 512];
        fs::read_bytes(INDEX_PATH, &mut buf, offset);
        buf
    }

    pub fn read_loc(loc: u64) -> Page {
        let offset: u64 = loc * 512;
        let mut buf = [0u8; 512];
        fs::read_bytes(INDEX_PATH, &mut buf, offset);
        buf
    }
}