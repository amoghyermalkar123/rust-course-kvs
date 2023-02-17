#![allow(dead_code)]

use std::{collections::HashMap};
pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        KvStore { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
       let ans = self.map.get(&key);
       ans.to_owned().cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
