use std::collections::{HashMap, HashSet};
use xxhash_rust::xxh3::xxh3_64;

pub enum Direction {
    Forward,
    Backward,
}

pub struct HashRing {
    replicas: usize,
    sorted_keys: Vec<u64>,
    hash_map: HashMap<u64, String>,
}

impl HashRing {
    pub fn new_with_replicas(replicas: usize) -> Self {
        HashRing {
            replicas,
            sorted_keys: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sorted_keys.is_empty()
    }

    fn generate_hash(&self, key: &str, i: usize) -> u64 {
      let mut s = String::with_capacity(key.len() + 10);
      s.push_str(key);
      s.push_str(i.to_string().as_str());
      xxh3_64(s.as_bytes())
    }

    pub fn add(&mut self, keys: &[String]) {
        for key in keys {
          for i in 0..self.replicas {
            let hash = self.generate_hash(key, i);
            self.sorted_keys.push(hash);
            self.hash_map.insert(hash, key.clone());
          }
        }

        // Sort the keys
        self.sorted_keys.sort_unstable();
    }

    pub fn remove(&mut self, key: &str) {
        let mut i = 0;
        while i < self.sorted_keys.len() {
          let hash_key = &self.hash_map[&self.sorted_keys[i]];
          if hash_key == key {
            let hash = self.sorted_keys.remove(i);
            self.hash_map.remove(&hash);
          } else {
            i += 1;
          }
        }
    }

    /// An optimized version of get_n that returns a single node.
    pub fn get(&self, key: &str) -> Option<&String> {
        if self.is_empty() {
          return None;
        }

        let hash = xxh3_64(key.as_bytes());
        let idx = match self.sorted_keys.binary_search(&hash) {
          // Exact match
          Ok(idx) => idx,
          // Not an exact match, get the first node that has a hash greater than the key hash
          Err(idx) => idx % self.sorted_keys.len(),
        };

        Some(&self.hash_map[&self.sorted_keys[idx]])
    }

    /// Returns the <=N unique nodes that are closest to the key in order.
    /// Useful for choosing replica groups for distributed systems (e.g. Raft group)
    /// By default, the nodes are returned in ascending order of the hash ring.
    pub fn get_n(&self, key: &str, n: usize, direction: Direction) -> Vec<String> {
      let mut result: Vec<String> = Vec::new();
      let mut _node_map:HashSet<String> = HashSet::new();

      if self.is_empty() || n == 0 {
          return result;
      }

      let hash = xxh3_64(key.as_bytes());
      let idx = match self.sorted_keys.binary_search(&hash) {
        // Exact match
        Ok(idx) => idx,
        // Not an exact match, get the first node that has a hash greater than the key hash
        Err(idx) => idx % self.sorted_keys.len(),
      };

      let mut current_idx = idx;
      while result.len() < n {
        let node = &self.hash_map[&self.sorted_keys[current_idx]];
        if _node_map.insert(node.clone()) {
          result.push(node.clone());
        }

        match direction {
            Direction::Forward => {
                current_idx = (current_idx + 1) % self.sorted_keys.len();
            }
            Direction::Backward => {
                if current_idx == 0 {
                    current_idx = self.sorted_keys.len() - 1;
                } else {
                    current_idx -= 1;
                }
            }
        }

        // Break if we hit the start point again (haven't found enough nodes)
        if current_idx == idx {
            break;
        }
      }

      result
    }

}
