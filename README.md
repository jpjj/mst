# Minimum Spanning Tree (MST)

A Rust implementation of Kruskal's algorithm for finding the Minimum Spanning Tree of a weighted undirected graph.

## Usage

```rust
use mst::{Graph, Node, Edge};

let mut graph = Graph::new(4);
graph.add_edge(0, 1, 10);
graph.add_edge(0, 2, 6);
graph.add_edge(0, 3, 5);
graph.add_edge(1, 3, 15);
graph.add_edge(2, 3, 4);

let (mst_edges, total_weight) = graph.kruskal_mst();
println!("MST total weight: {}", total_weight);
```

## Features

- Efficient Kruskal's algorithm implementation
- Union-Find data structure with path compression and union by rank
- Support for disconnected graphs
- Comprehensive test suite