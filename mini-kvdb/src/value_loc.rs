use crate::offset::WOffset;

pub struct ValueLoc {
   pub offset: u64,
    pub len: u32,
}

impl ValueLoc {
    pub fn new(offset: u64, len: u32) -> Self {
        Self { offset, len }
    }

    pub fn from_w_offset(w_offset: WOffset, value_len: u32) -> Self {
        let offset = w_offset.offset + w_offset.len as u64 - value_len as u64;
        Self { offset, len: value_len }
    }
}