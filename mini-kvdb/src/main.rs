use std::path::PathBuf;
use mini_bitcask_kvdb::kvdb::MiniKVDB;

fn main() {
    let path = PathBuf::from("mini.db");
    let mut db = MiniKVDB::new(path).unwrap();

    let v = vec![10,20,30,40,50];
    let key = b"key1";
    let _ = db.set(&key[..], v.clone());
    let ret = db.get(&key[..]).unwrap().unwrap();
    assert_eq!(ret,v);
}
