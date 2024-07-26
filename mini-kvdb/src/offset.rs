/**
 * 写入文件的offset 和写入内容的长度
 */
pub struct WOffset {
   pub offset: u64,
    pub len: u32,
}

impl WOffset {
    pub fn new(offset: u64, len: u32) -> Self {
        Self { offset, len }
    }
}