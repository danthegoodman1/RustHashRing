use basic_hash_ring::{Direction, HashRing};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("HashRing Consistency Test");
    println!("=========================\n");

    // Create a new hash ring with default settings
    let start = Instant::now();
    // let mut hash_ring = HashRing::new();
    let mut hash_ring = HashRing::new_with_replicas(100);
    println!("Hash ring initialization: {:?}", start.elapsed());

    // Test 1: Basic Distribution
    println!("Test 1: Basic distribution with 3 nodes");
    let nodes = vec![
        "node1.example.com".to_string(),
        "node2.example.com".to_string(),
        "node3.example.com".to_string(),
    ];

    let start = Instant::now();
    hash_ring.add(&nodes);
    println!("Adding 3 nodes: {:?}", start.elapsed());

    // Generate a large number of keys to test distribution
    let mut distribution: HashMap<String, usize> = HashMap::new();

    // Test with a large set of keys
    let start = Instant::now();
    for i in 0..1000 {
        let key = format!("test-key-{}", i);
        if let Some(node) = hash_ring.get(&key) {
            let node_name = node.clone();
            *distribution.entry(node_name).or_insert(0) += 1;
        }
    }
    println!("Lookup of 1000 keys: {:?}", start.elapsed());

    // Print distribution statistics
    println!("Node distribution with 1000 keys:");
    let total_keys = distribution.values().sum::<usize>();
    for (node, count) in distribution.iter() {
        let percentage = (*count as f64 / total_keys as f64) * 100.0;
        println!("  {}: {} keys ({:.2}%)", node, count, percentage);
    }
    println!();

    // Test 2: Consistency after node addition
    println!("Test 2: Consistency after adding a node");

    // Store the current mapping for a sample of keys
    let sample_keys: Vec<String> = (0..100).map(|i| format!("sample-key-{}", i)).collect();
    let mut before_addition: HashMap<String, String> = HashMap::new();

    for key in &sample_keys {
        if let Some(node) = hash_ring.get(key) {
            before_addition.insert(key.clone(), node.clone());
        }
    }

    // Add a new node
    let start = Instant::now();
    hash_ring.add(&[String::from("node4.example.com")]);
    println!("Adding 1 node: {:?}", start.elapsed());

    // Check how many keys were remapped
    let mut remapped = 0;
    for key in &sample_keys {
        if let Some(node) = hash_ring.get(key) {
            if let Some(before) = before_addition.get(key) {
                if before != node {
                    remapped += 1;
                }
            }
        }
    }

    println!("After adding node4.example.com:");
    println!(
        "  {} out of {} keys were remapped ({:.2}%)",
        remapped,
        sample_keys.len(),
        (remapped as f64 / sample_keys.len() as f64) * 100.0
    );

    // Test 3: Consistency after node removal
    println!("\nTest 3: Consistency after removing a node");

    // Store the current mapping
    let mut before_removal: HashMap<String, String> = HashMap::new();

    for key in &sample_keys {
        if let Some(node) = hash_ring.get(key) {
            before_removal.insert(key.clone(), node.clone());
        }
    }

    // Remove a node
    let start = Instant::now();
    hash_ring.remove("node1.example.com");
    println!("Removing 1 node: {:?}", start.elapsed());

    // Check how many keys were remapped
    let mut remapped = 0;
    for key in &sample_keys {
        if let Some(node) = hash_ring.get(key) {
            if let Some(before) = before_removal.get(key) {
                if before != node {
                    remapped += 1;
                }
            }
        }
    }

    println!("After removing node1.example.com:");
    println!(
        "  {} out of {} keys were remapped ({:.2}%)",
        remapped,
        sample_keys.len(),
        (remapped as f64 / sample_keys.len() as f64) * 100.0
    );

    // Test 4: Different key patterns
    println!("\nTest 4: Different key patterns");

    let start = Instant::now();
    let mut hash_ring = HashRing::new_with_replicas(100);
    hash_ring.add(&nodes);
    println!("Reinitializing and adding 3 nodes: {:?}", start.elapsed());

    // Test with different key patterns
    let patterns = vec![
        "user:1234",
        "user:5678",
        "user:9012",
        "post:1001",
        "post:1002",
        "post:1003",
        "image:large:1",
        "image:medium:2",
        "image:small:3",
        "session:abcdef",
        "session:ghijkl",
        "session:mnopqr",
    ];

    let start = Instant::now();
    println!("Mapping of different key patterns:");
    for key in patterns {
        if let Some(node) = hash_ring.get(key) {
            println!("  {} -> {}", key, node);
        }
    }
    println!("Lookup of 12 different pattern keys: {:?}", start.elapsed());

    println!();

    // Get N nodes
    let start = Instant::now();
    let n_nodes = hash_ring.get_n("test-key", 3, Direction::Forward);
    println!("Get 3 nodes (forward): {:?}", start.elapsed());
    println!("Nodes: {:?}", n_nodes);

    let start = Instant::now();
    let n_nodes = hash_ring.get_n("test-key", 3, Direction::Backward);
    println!("Get 3 nodes (backward): {:?}", start.elapsed());
    println!("Nodes: {:?}", n_nodes);

    // Benchmark a large number of lookups
    let start = Instant::now();
    for i in 0..100_000 {
        let key = format!("benchmark-key-{}", i);
        let _ = hash_ring.get(&key);
    }
    println!("\nBenchmark: 100,000 key lookups: {:?}", start.elapsed());
}
