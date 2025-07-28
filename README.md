This project is about implementing an efficient mst algorithm.
Prerequisite is beinggg especially efficient when called multiple times on a graph with slightly changing weights.

Top candidate for this is Kruskal's algorithm, whose bottleneck lies in the sorting. If the partly sorted array of edges can be given as an input, the algorithm might be exceptionally fast.


Having created some benchmarks, it can be seen that when running on partly sorted arrays, the overall algorithm can be significantly faster.
These benchmark results are for a graph with 2000 nodes and 40000 edges with random weights:


| Task | Not sorted | 90% sorted |
| -------- | ------- | ------ |
| Sorting | 544.00 µs | 55.320 µs |
| Union-Find | 73.682 µs | 72.883 µs |
| Complete | 602.09 µs | 115.27 µs |

### Observations
- Sorting is indeed the bottleneck but gets much faster if array is partly sorted. Here, 100 random swaps where executed, leaving roughly 90% of the array untouched.
- Own implementation of Union-Find data structure has been compared with the one of this [crate](https://github.com/gifnksm/union-find-rs). Own implementation was slightly faster on benchmark.
   - Here, decisions like implementing `find` iteratively let to excellent time saves. Deciding whether to use union on rank or union on size did not really change the results in the benchmark. Union by rank was finally used just as a matter of taste. 
   - Unsafe code was added to find method but not to union method. Only the former improved performance.
