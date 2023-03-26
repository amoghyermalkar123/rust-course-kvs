#![deny(missing_docs)]
#![allow(dead_code)]

//! this is the kv store implementation
//! stores data in memory
mod constants;
mod db;
mod wal;
use db::DB;

/// KvStore is the memory store
pub struct KvStore {
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
        let mut db = DB::new()?;
        db.load_indexes()?;
        Ok(KvStore { db })
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
        self.db.insert(key, val)?;
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
            self.db.get(key).ok()
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
        self.db.del(key)?;
        Ok(())
    }
}
