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
    pub fn new() -> Self {
        KvStore { map: HashMap::new() }
    }

    /// insert the key value pair in memory
    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    /// get the value by key
    pub fn get(&self, key: String) -> Option<String> {
       let ans = self.map.get(&key);
       ans.to_owned().cloned()
    }

    /// remove a key value pair, accepts key
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
