use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use mst::Graph;

fn create_sparse_graph(n: usize, edges_per_node: usize) -> Graph {
    let mut graph = Graph::new();
    let mut weight = 1;

    for i in 0..n {
        for j in 0..edges_per_node.min(n - i - 1) {
            let target = i + j + 1;
            if target < n {
                graph.add_edge(i, target, weight);
                weight = rand::random_range(1..=1000);
            }
        }
    }

    graph
}

fn benchmark_kruskal(c: &mut Criterion) {
    let mut group = c.benchmark_group("kruskal");

    for size in [2000].iter() {
        group.bench_with_input(BenchmarkId::new("sorting", size), &size, |b, &s| {
            b.iter_batched(
                || create_sparse_graph(*s, 20),
                |mut graph| black_box(graph.sort_edges()),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("union-find", size), &size, |b, &s| {
            b.iter_batched(
                || {
                    let mut graph = create_sparse_graph(*s, 20);
                    graph.sort_edges();
                    graph
                },
                |graph| black_box(graph.kruskal_from_sorted_edges()),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("both", size), &size, |b, &s| {
            b.iter_batched(
                || create_sparse_graph(*s, 20),
                |mut graph| black_box(graph.kruskal_mst()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn benchmark_kruskal_partly_sorted(c: &mut Criterion) {
    let mut group = c.benchmark_group("kruskal");

    for size in [2000].iter() {
        group.bench_with_input(BenchmarkId::new("sorting", size), &size, |b, &s| {
            b.iter_batched(
                || {
                    let mut graph = create_sparse_graph(*size, 20);
                    graph.edges.sort_by_key(|x| x.weight);
                    for _ in 0..2000 {
                        graph
                            .edges
                            .swap(rand::random_range(0..2000), rand::random_range(0..2000));
                    }
                    graph
                },
                |mut graph| black_box(graph.sort_edges()),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("union-find", size), &size, |b, &s| {
            b.iter_batched(
                || {
                    let mut graph = create_sparse_graph(*s, 20);
                    graph.sort_edges();
                    graph
                },
                |graph| black_box(graph.kruskal_from_sorted_edges()),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("both", size), &size, |b, &s| {
            b.iter_batched(
                || {
                    let mut graph = create_sparse_graph(*size, 20);
                    graph.edges.sort_by_key(|x| x.weight);
                    for _ in 0..100 {
                        graph
                            .edges
                            .swap(rand::random_range(0..2000), rand::random_range(0..2000));
                    }
                    graph
                },
                |mut graph| black_box(graph.kruskal_mst()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_kruskal, benchmark_kruskal_partly_sorted);
criterion_main!(benches);
