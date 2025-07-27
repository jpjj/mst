This project is for implementing an efficient mst algorithm.
Prerequisite is that it is especially efficient being called multiple times on graphs with slightly changing weights.

Top candidate for this is Kruskal's algorithm, whose bottleneck lies in the sorting. If the partly sorted array of edges can be given as an input, the algorithm might be exceptionally fast.


Having created some benchmarks, it can be seen that when running on partly sorted arrays, the overall algorithm can be significantly faster.
These benchmark results are for a graph with 2000 nodes and 20 random edges for each node with random weights assigned:


| Task | Not sorted | 95% sorted |
| -------- | ------- | ------ |
| Sorting | 547.48 µs | 35.833 µs |
| Union-Find | 77.089 µs | 83.118 µs |
| Complete | 647.53 µs | 94.919 µs |

### Obserations
- Sorting is indeed the bottleneck but gets much faster if array is partly sorted. Here, the first 100 entries where shuffled.
- Own implementation of Union-Find data structure has been compared with the one of this [crate](https://github.com/gifnksm/union-find-rs). Own implementation was slightly faster on benchmark.
   - Here, decisions like implementing `find` iteratively let to excellent time saves. Deciding whether to use union on rank or union on size did not really change the results in the benchmark. Union by rank was finally used just as a matter of taste. 
