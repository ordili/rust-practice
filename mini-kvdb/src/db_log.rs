use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use crate::offset::WOffset;

const KEY_VAL_HEADER_LEN: u32 = 4;

pub struct DbLog {
    path: PathBuf,
    file: std::fs::File,
}

impl DbLog {
    pub fn new(path: PathBuf) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)
            .unwrap();
        DbLog { path, file }
    }


    /**
     *
     *  Read value from the fixed pos.
     */
    pub fn read_value(&mut self, value_offset: u64, value_len: usize) -> crate::kvdb::Result<Vec<u8>> {
        let mut value = vec![0u8; value_len];
        self.file.seek(SeekFrom::Start(value_offset))?;
        self.file.read_exact(&mut value);
        Ok(value)
    }

    /**
     * format : key_len + value_len + key + value
     */
    pub fn write_entry(&mut self, key: &[u8], value: Option<&[u8]>) -> crate::kvdb::Result<WOffset> {

        let key_len = key.len() as u32;
        let value_len = value.map_or(0, |v| v.len() as u32);
        let value_len_or_tomestone = value.map_or(-1, |v| v.len() as i32);

        // 总共占据的长度
        let len = KEY_VAL_HEADER_LEN * 2 + key_len + value_len;

        let offset = self.file.seek(SeekFrom::End(0))?;

        let mut w = BufWriter::with_capacity(len as usize, &mut self.file);
        w.write_all(&key_len.to_be_bytes())?;
        w.write_all(&value_len_or_tomestone.to_be_bytes())?;
        w.write_all(key)?;
        if let Some(value) = value {
            w.write_all(value)?;
        }
        w.flush()?;

        let w_offset = WOffset::new(offset, len as u32);
        Ok(w_offset)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::db_log::DbLog;
    use crate::value_loc::ValueLoc;

    #[test]
    fn test_new() {
        let path = PathBuf::from("abc.txt");
        let log = DbLog::new(path);
    }

    #[test]
    fn test_read_write() {
        let path = PathBuf::from("abc.txt");
        let mut log = DbLog::new(path);

        let key = b"abc";
        let val: Vec<u8> = vec![1, 2, 3, 4, 5];

        let w_offset = log.write_entry(key, Some(&val.clone())).unwrap();

        let val_loc = ValueLoc::from_w_offset(w_offset, val.len() as u32);

        let ret = log.read_value(val_loc.offset, val_loc.len as usize).unwrap();

        assert_eq!(val, ret);
    }
}