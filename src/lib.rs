#![deny(missing_docs)]
#![allow(dead_code)]

//! this is the kv store implementation
//! stores data in memory
mod constants;
mod db;
mod wal;
use db::Meta;
use db::DB;
use std::collections::HashMap;

/// KvStore is the memory store
pub struct KvStore {
    map: HashMap<String, Meta>,
    db: DB,
}

/// KvStore methods
impl KvStore {
    /// creates a new instance of the memory store
    ///
    /// # Example
    ///
    /// ```rust
    ///  let mut kvs = kvs::KvStore::new();
    /// // your logic here
    /// ```
    pub fn new() -> anyhow::Result<Self> {
        let db = DB::new()?;
        let in_mem_kv = db.load_indexes(HashMap::new())?;
        Ok(KvStore { map: in_mem_kv, db })
    }

    /// insert the key value pair in memory
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut kvs = kvs::KvStore::new();
    /// let key = "key".to_owned();
    /// let val = "val".to_owned();
    /// kvs.set(key.to_owned(), val.to_owned())
    /// ```
    pub fn set(&mut self, key: String, val: String) -> anyhow::Result<()> {
        let meta = self.db.insert(wal::WALEntry {
            key_size: key.as_bytes().len(),
            value_size: val.as_bytes().len(),
            key: key.as_bytes(),
            value: val.as_bytes(),
        })?;
        self.map.insert(key, meta);
        Ok(())
    }

    /// get the value by key
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut kvs = kvs::KvStore::new();
    /// let val = "val".to_owned();
    /// kvs.get(val.to_owned());
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        if let Some(ans) = self.map.get(&key) {
            self.db.get(ans).ok()
        } else {
            None
        }
    }

    /// remove a key value pair, accepts key
    ///
    /// # Example
    /// ```rust
    /// let mut kvs = kvs::KvStore::new();
    /// let key = "key".to_owned();
    /// kvs.remove(key.to_owned());
    /// ```
    pub fn remove(&mut self, key: String) -> anyhow::Result<()> {
        self.db.del(wal::WALEntry {
            key_size: key.as_bytes().len(),
            value_size: 0,
            key: key.as_bytes(),
            value: "".as_bytes(),
        })?;
        self.map.remove(&key);
        Ok(())
    }
}
