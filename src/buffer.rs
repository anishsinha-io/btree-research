use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex, RwLock};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use crate::Page;


#[derive(Debug, Clone)]
pub struct BufferPoolFrame {
    pub pins: u32,
    pub page_no_held: u32,
    pub dirty: bool,
    pub page_held: Page,
}

impl Display for BufferPoolFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferPoolFrame [pins={} page_no_held={} dirty={}]", self.pins, self.page_no_held, self.dirty)
    }
}

impl BufferPoolFrame {
    pub fn new(page_no: u32, page: Page) -> Self {
        BufferPoolFrame {
            pins: 0,
            page_no_held: page_no,
            dirty: false,
            page_held: page,
        }
    }
}

type BufferPool = Arc<RwLock<Vec<Arc<RwLock<BufferPoolFrame>>>>>;

lazy_static! {
   pub static ref GLOBAL_BUFFER_POOL: BufferPool = Arc::new(RwLock::new(Vec::new()));
}

type FrameResult = Result<u32, String>;

pub fn add_frame(page_no: u32, page: Page) -> FrameResult {
    // acquire write lock on global buffer pool
    if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
        let mut new_frame = BufferPoolFrame::new(page_no, page);
        new_frame.pins = 1;
        let size = pages.len();
        pages.insert(size, Arc::new(RwLock::new(new_frame)));
        return Ok(page_no);
    }
    Err("could not add frame to buffer pool".to_string())
}

pub fn drop_frame(page_no: u32) -> FrameResult {
    if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
        pages.remove(page_no as usize);
        return Ok(page_no);
    }
    Err("could not remove frame from buffer pool".to_string())
}

pub fn fetch_frame(page_no: u32) -> Option<BufferPoolFrame> {
    if let Ok(pages) = GLOBAL_BUFFER_POOL.read() {
        if let Some(frame) = pages.get(page_no as usize) {
            if let Ok(locked) = frame.read() {
                return Some((*locked).clone());
            }
        }
    }
    None
}
