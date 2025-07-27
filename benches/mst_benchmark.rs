use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mst::Graph;
use rand::seq::SliceRandom;


fn create_sparse_graph(n: usize, edges_per_node: usize) -> Graph {
    let mut graph = Graph::new(n);
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
        let graph = create_sparse_graph(*size, 20);
        let sorted_edges = graph.sort_edges();
        group.bench_with_input(
            BenchmarkId::new("sorting", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(graph.sort_edges())
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("union-find", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(graph.kruskal_from_sorted_edges(sorted_edges.clone()))
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("both", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(graph.kruskal_mst())
                });
            },
        );

    }
    
    group.finish();
}

fn benchmark_kruskal_partly_sorted(c: &mut Criterion) {
    let mut group = c.benchmark_group("kruskal");
    
    for size in [2000].iter() {
        let mut graph = create_sparse_graph(*size, 20);
        graph.edges.sort_by_key(|x| x.weight);
        graph.edges[0..100].shuffle(&mut rand::rng());
        let sorted_edges = graph.sort_edges();
        group.bench_with_input(
            BenchmarkId::new("sorting", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(graph.sort_edges())
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("union-find", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(graph.kruskal_from_sorted_edges(sorted_edges.clone()))
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("both", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(graph.kruskal_mst())
                });
            },
        );

    }
    
    group.finish();
}



criterion_group!(
    benches,
    benchmark_kruskal,
    benchmark_kruskal_partly_sorted
);
criterion_main!(benches);