#![deny(missing_docs)]
#![allow(dead_code)]

//! this is the kv store implementation
//! stores data in memory

use std::collections::HashMap;

/// KvStore is the memory store
pub struct KvStore {
    map: HashMap<String, String>
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
    pub fn new() -> Self {
        KvStore { map: HashMap::new() }
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
    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
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
       let ans = self.map.get(&key);
       ans.to_owned().cloned()
    }

    /// remove a key value pair, accepts key
    /// 
    /// # Example
    /// ```rust
    /// let mut kvs = kvs::KvStore::new();
    /// let key = "key".to_owned();
    /// kvs.remove(key.to_owned());
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
