use std::collections::BTreeMap;
use std::path::PathBuf;
use crate::db_log::DbLog;
use crate::value_loc::ValueLoc;

pub type Result<T> = std::result::Result<T, std::io::Error>;


pub struct MiniKVDB {
    log: DbLog,
    key_index: BTreeMap<Vec<u8>, ValueLoc>,
}

impl MiniKVDB {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut log = DbLog::new(path);
        // to do, we need load the key_index from the file.
        let key_index = BTreeMap::new();
        Ok(Self { log, key_index })
    }

    pub fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        let w_offset = self.log.write_entry(key, Some(&value))?;
        let val_loc = ValueLoc::from_w_offset(w_offset, value.len() as u32);
        self.key_index.insert(key.to_vec(), val_loc);
        Ok(())
    }

    pub fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        if let Some(val_loc) = self.key_index.get(key) {
            let val = self.log.read_value(val_loc.offset, val_loc.len as usize)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }
}

