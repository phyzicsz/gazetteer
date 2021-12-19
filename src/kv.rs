use rocksdb::DB;
use std::sync::Arc;

pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn insert(&self, k: &str, v: &str) -> bool;
    fn lookup(&self, k: &str) -> Option<String>;
    //fn delete(&self, k: &str) -> bool;
}

#[derive(Clone)]
pub struct RocksDB {
    db: Arc<DB>,
}

impl KVStore for RocksDB {
    fn init(file_path: &str) -> Self {
        RocksDB { db: Arc::new(DB::open_default(file_path).unwrap()) }
    }

    fn insert(&self, k: &str, v: &str) -> bool {
        self.db.put(k.as_bytes(), v.as_bytes()).is_ok()
    }

    fn lookup(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v).unwrap();
                Some(result)
            },
            Ok(None) => {
                None
            },
            Err(e) => {
                error!("Error with lookup: {}", e);
                None
            }
        }
    }
}
