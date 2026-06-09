use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use best_path::prelude::*;
use best_path::prelude::floyd_warshall::calculator::{FloydWarshallCalculator, SCALE};
use best_path::prelude::bellman_ford::calculator::BellmanFordCalculator;

/// Ring topology: C0->C1->C2->...->C(n-1)->C0.
/// Rates are all < 1.0 so -log2(rate) > 0 on every edge; no cycle can accumulate a
/// negative cost, satisfying the algorithm's negative-cycle precondition.
/// Each edge is replicated once per provider.
fn make_sparse_graph(n_nodes: usize, n_providers: usize) -> Vec<(ProviderPair<String, String>, u128)> {
    let currencies: Vec<String> = (0..n_nodes).map(|i| format!("C{i:03}")).collect();
    let providers: Vec<String> = (0..n_providers).map(|i| format!("P{i}")).collect();
    let mut graph = Vec::new();
    for i in 0..n_nodes {
        // rates in (0.5, 0.6) — all < 1.0
        let cost = ((0.5 + i as f64 * 0.001) * SCALE) as u128;
        for provider in &providers {
            graph.push((
                ProviderPair {
                    pair: Pair { source: currencies[i].clone(), target: currencies[(i + 1) % n_nodes].clone() },
                    provider: provider.clone(),
                },
                cost,
            ));
        }
    }
    graph
}

/// All-pairs topology: every node connects to every other node (N*(N-1) directed edges).
/// Same rate constraint as make_sparse_graph.
fn make_dense_graph(n_nodes: usize, n_providers: usize) -> Vec<(ProviderPair<String, String>, u128)> {
    let currencies: Vec<String> = (0..n_nodes).map(|i| format!("C{i:03}")).collect();
    let providers: Vec<String> = (0..n_providers).map(|i| format!("P{i}")).collect();
    let mut graph = Vec::new();
    for i in 0..n_nodes {
        for j in 0..n_nodes {
            if i == j { continue; }
            // rates in (0.1, 0.6) — all < 1.0
            let cost = ((0.1 + (i * n_nodes + j) as f64 / (n_nodes * n_nodes) as f64 * 0.5) * SCALE) as u128;
            for provider in &providers {
                graph.push((
                    ProviderPair {
                        pair: Pair { source: currencies[i].clone(), target: currencies[j].clone() },
                        provider: provider.clone(),
                    },
                    cost,
                ));
            }
        }
    }
    graph
}

/// Measures how runtime scales with node count (O(V³) expected).
fn bench_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("size_scaling");
    group.sample_size(10);
    for nodes in [10usize, 20, 30, 40, 50] {
        let graph = make_sparse_graph(nodes, 1);
        group.bench_with_input(BenchmarkId::new("sparse_1p", nodes), &graph, |b, g| {
            b.iter(|| FloydWarshallCalculator::calc_best_paths(g.as_slice()).unwrap());
        });
    }
    group.finish();
}

/// Measures how extra providers per pair affect runtime (deduplication in best_edge_per_pair).
fn bench_provider_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("provider_scaling");
    group.sample_size(10);
    for providers in [1usize, 3, 5] {
        let graph = make_sparse_graph(30, providers);
        group.bench_with_input(BenchmarkId::new("30nodes", providers), &graph, |b, g| {
            b.iter(|| FloydWarshallCalculator::calc_best_paths(g.as_slice()).unwrap());
        });
    }
    group.finish();
}

/// Compares sparse (ring, N edges) vs dense (all-pairs, N*(N-1) edges) at 30 nodes.
fn bench_edge_density(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_density");
    group.sample_size(10);
    let sparse = make_sparse_graph(30, 1);
    group.bench_function("sparse_30n_1p", |b| {
        b.iter(|| FloydWarshallCalculator::calc_best_paths(sparse.as_slice()).unwrap());
    });
    let dense = make_dense_graph(30, 1);
    group.bench_function("dense_30n_1p", |b| {
        b.iter(|| FloydWarshallCalculator::calc_best_paths(dense.as_slice()).unwrap());
    });
    group.finish();
}

/// Bellman-Ford single-source: same node sizes as size_scaling for direct comparison.
/// BF is O(VE) per source; FW is O(V³) for all pairs.
fn bench_bf_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("bf_size_scaling");
    group.sample_size(10);
    for nodes in [10usize, 20, 30, 40, 50] {
        let graph = make_sparse_graph(nodes, 1);
        let source = format!("C{:03}", 0);
        group.bench_with_input(BenchmarkId::new("sparse_1p", nodes), &graph, |b, g| {
            b.iter(|| BellmanFordCalculator::calc_best_paths_from(&source, g.as_slice()).unwrap());
        });
    }
    group.finish();
}

/// Floyd-Warshall (all-pairs) vs Bellman-Ford (single-source) on sparse ring graphs.
/// Highlights the per-source cost difference: FW amortizes V sources; BF computes one.
fn bench_fw_vs_bf(c: &mut Criterion) {
    let mut group = c.benchmark_group("fw_vs_bf");
    group.sample_size(10);
    for nodes in [10usize, 20, 30] {
        let graph = make_sparse_graph(nodes, 1);
        let source = format!("C{:03}", 0);
        group.bench_with_input(BenchmarkId::new("fw_all_pairs", nodes), &graph, |b, g| {
            b.iter(|| FloydWarshallCalculator::calc_best_paths(g.as_slice()).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("bf_single_source", nodes), &graph, |b, g| {
            b.iter(|| BellmanFordCalculator::calc_best_paths_from(&source, g.as_slice()).unwrap());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_size_scaling, bench_provider_scaling, bench_edge_density, bench_bf_size_scaling, bench_fw_vs_bf);
criterion_main!(benches);
