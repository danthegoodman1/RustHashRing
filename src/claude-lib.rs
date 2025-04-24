/**
 * This one was claude-generated from https://github.com/golang/groupcache/blob/master/consistenthash/consistenthash.go as a reference
 */

use std::collections::HashMap;
use xxhash_rust::xxh3::xxh3_64;

// Hash function type
pub type HashFn = fn(&[u8]) -> u64;

pub struct HashRing {
    replicas: usize,
    hash: HashFn,
    keys: Vec<u64>, // Sorted
    hash_map: HashMap<u64, String>,
}

impl HashRing {
    pub fn new() -> Self {
        HashRing::with_hash_and_replicas(100, xxh3_64)
    }

    pub fn with_hash_and_replicas(replicas: usize, hash_fn: HashFn) -> Self {
        HashRing {
            replicas,
            hash: hash_fn,
            keys: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn add(&mut self, keys: &[String]) {
        for key in keys {
            for i in 0..self.replicas {
                // Create a unique hash key for each replica by combining the replica number and node name
                let hash_key = format!("{}{}", i, key);
                let hash = (self.hash)(hash_key.as_bytes());
                self.keys.push(hash);
                self.hash_map.insert(hash, key.clone());
            }
        }
        self.keys.sort_unstable();
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        if self.is_empty() {
            return None;
        }

        let hash = (self.hash)(key.as_bytes());
        
        // Using a binary search to find the proper position
        let idx = match self.keys.binary_search(&hash) {
            // Exact match
            Ok(idx) => idx,
            // No exact match, find the first node that has a hash greater than the key hash
            Err(idx) => idx % self.keys.len(),
        };
        
        Some(&self.hash_map[&self.keys[idx]])
    }
    
    // Remove a node from the ring
    pub fn remove(&mut self, key: &str) {
        let mut i = 0;
        while i < self.keys.len() {
            let hash_key = &self.hash_map[&self.keys[i]];
            if hash_key == key {
                let hash = self.keys.remove(i);
                self.hash_map.remove(&hash);
            } else {
                i += 1;
            }
        }
    }
}
