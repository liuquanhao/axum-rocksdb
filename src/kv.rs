use std::sync::Arc;
use rocksdb::DB;
use axum::async_trait;

pub struct RocksDB {
    db: Arc<DB>,
}

#[async_trait]
pub trait KVStore {
    async fn save(&self, k: &str, v: &str) -> bool;
    async fn find(&self, k: &str) -> Option<String>;
    async fn delete(&self, k:&str) -> bool;
}

#[async_trait]
impl KVStore for RocksDB {
    async fn save(&self, k: &str, v: &str) -> bool {
        self.db.put(k.as_bytes(), v.as_bytes()).is_ok()
    }

    async fn find(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let val = String::from_utf8(v).unwrap();
                Some(val)
            },
            _ => None
        }
    }

    async fn delete(&self, k: &str) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }
}

impl RocksDB {
    pub async fn new(path: &str) -> Self {
        RocksDB {
            db: Arc::new(DB::open_default(path).unwrap())
        }
    }
}