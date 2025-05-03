# basic_hash_ring

[docs.rs](https://docs.rs/basic_hash_ring/latest/basic_hash_ring/) [crates.io](https://crates.io/crates/basic_hash_ring)

Mostly a Rust impl of https://github.com/golang/groupcache/blob/master/consistenthash/consistenthash.go

```
cargo run --release
```

On an M3 Max 128GB:

```
HashRing Consistency Test
=========================

Hash ring initialization: 1.583µs
Test 1: Basic distribution with 3 nodes
Adding 3 nodes: 90.5µs
Lookup of 1000 keys: 186.5µs
Node distribution with 1000 keys:
  node1.example.com: 358 keys (35.80%)
  node3.example.com: 312 keys (31.20%)
  node2.example.com: 330 keys (33.00%)

Test 2: Consistency after adding a node
Adding 1 node: 18.375µs
After adding node4.example.com:
  22 out of 100 keys were remapped (22.00%)

Test 3: Consistency after removing a node
Removing 1 node: 24.958µs
After removing node1.example.com:
  27 out of 100 keys were remapped (27.00%)

Test 4: Different key patterns
Reinitializing and adding 3 nodes: 57.125µs
Mapping of different key patterns:
  user:1234 -> node2.example.com
  user:5678 -> node1.example.com
  user:9012 -> node2.example.com
  post:1001 -> node3.example.com
  post:1002 -> node3.example.com
  post:1003 -> node3.example.com
  image:large:1 -> node2.example.com
  image:medium:2 -> node2.example.com
  image:small:3 -> node1.example.com
  session:abcdef -> node2.example.com
  session:ghijkl -> node1.example.com
  session:mnopqr -> node1.example.com
Lookup of 12 different pattern keys: 15µs

Get 3 nodes (forward): 18.166µs
Nodes: ["node2.example.com", "node3.example.com", "node1.example.com"]
Get 3 nodes (backward): 666ns
Nodes: ["node2.example.com", "node1.example.com", "node3.example.com"]

Benchmark: 100,000 key lookups: 9.65425ms
```
