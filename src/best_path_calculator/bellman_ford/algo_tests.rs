use super::algo::*;
use core::cmp::Ordering;
#[cfg(not(feature = "std"))]
use alloc::vec;

/// Simple chain: 0→1→2, verify direct and multi-hop paths from source 0.
#[test]
fn test_simple_chain() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 0, cost: 3.0 },
        Edge { pair: Pair { source: 1, target: 2 }, provider: 0, cost: 2.0 },
    ];
    let res = bellman_ford(&edges, 0).unwrap();

    assert_eq!(res[&0].total_cost, 0.0);
    assert_eq!(res[&0].edges, vec![]);

    assert_eq!(res[&1].total_cost, 3.0);
    assert_eq!(res[&1].edges, vec![edges[0]]);

    assert_eq!(res[&2].total_cost, 5.0);
    assert_eq!(res[&2].edges, vec![edges[0], edges[1]]);

    // vertex 2 cannot reach anything further — only 3 entries
    assert_eq!(res.len(), 3);
}

/// Two paths to the same target: indirect should win when cheaper.
#[test]
fn test_prefers_cheaper_path() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 2 }, provider: 0, cost: 10.0 }, // direct but expensive
        Edge { pair: Pair { source: 0, target: 1 }, provider: 0, cost: 3.0 },
        Edge { pair: Pair { source: 1, target: 2 }, provider: 0, cost: 2.0 },  // indirect but cheaper
    ];
    let res = bellman_ford(&edges, 0).unwrap();
    assert_eq!(res[&2].total_cost, 5.0);
    assert_eq!(res[&2].edges.len(), 2); // went via 1
}

/// Source not in the graph returns empty.
#[test]
fn test_unknown_source() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 0, cost: 1.0 },
    ];
    let res = bellman_ford(&edges, 99).unwrap();
    assert!(res.is_empty());
}

/// Vertices unreachable from source are absent.
#[test]
fn test_unreachable_vertices_absent() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 0, cost: 1.0 },
        Edge { pair: Pair { source: 2, target: 3 }, provider: 0, cost: 1.0 }, // disconnected component
    ];
    let res = bellman_ford(&edges, 0).unwrap();
    assert!(res.contains_key(&0));
    assert!(res.contains_key(&1));
    assert!(!res.contains_key(&2));
    assert!(!res.contains_key(&3));
}

/// Negative cycle is detected.
#[test]
fn test_negative_cycle() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 0, cost:  3.0 },
        Edge { pair: Pair { source: 1, target: 2 }, provider: 0, cost:  2.0 },
        Edge { pair: Pair { source: 2, target: 0 }, provider: 0, cost: -6.0 }, // cycle sum = -1
    ];
    assert!(bellman_ford(&edges, 0).is_err());
}

/// best_edge_per_pair: Ordering::Less keeps max, Ordering::Greater keeps min.
#[test]
fn test_unique_edges() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 1, cost: 1.0 },
        Edge { pair: Pair { source: 0, target: 1 }, provider: 2, cost: 5.0 },
        Edge { pair: Pair { source: 0, target: 1 }, provider: 3, cost: 3.0 },
    ];
    // Ordering::Less keeps max
    let deduped = best_edge_per_pair(&edges, Ordering::Less);
    assert_eq!(deduped.len(), 1);
    assert_eq!(deduped[0].cost, 5.0);

    // Ordering::Greater keeps min
    let deduped = best_edge_per_pair(&edges, Ordering::Greater);
    assert_eq!(deduped.len(), 1);
    assert_eq!(deduped[0].cost, 1.0);
}

/// Same graph as the Floyd-Warshall youtube test — verify distances from source 0.
///
///         8            3
///    0. *----------------* 1.
///    * *                   |
///  2 |  5 \                |
///    |       \             |
///    |          \          |
///    |             \       |
///  7 |                \    | 2
///    *                   \ *
///    3. *----------------- 2.
///         1
///
/// Expected distances from 0: {0:0, 1:3, 2:5, 3:6}
#[test]
fn test_youtube_distances() {
    let edges = vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 1, cost: 3.0 },
        Edge { pair: Pair { source: 0, target: 3 }, provider: 1, cost: 7.0 },
        Edge { pair: Pair { source: 1, target: 0 }, provider: 1, cost: 8.0 },
        Edge { pair: Pair { source: 1, target: 2 }, provider: 1, cost: 2.0 },
        Edge { pair: Pair { source: 2, target: 0 }, provider: 1, cost: 5.0 },
        Edge { pair: Pair { source: 2, target: 3 }, provider: 1, cost: 1.0 },
        Edge { pair: Pair { source: 3, target: 0 }, provider: 1, cost: 2.0 },
    ];
    let res = bellman_ford(&edges, 0).unwrap();
    assert_eq!(res[&0].total_cost, 0.0);
    assert_eq!(res[&1].total_cost, 3.0);
    assert_eq!(res[&2].total_cost, 5.0);
    assert_eq!(res[&3].total_cost, 6.0);

    // path to 3 should be 0→1→2→3
    assert_eq!(res[&3].edges, vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 1, cost: 3.0 },
        Edge { pair: Pair { source: 1, target: 2 }, provider: 1, cost: 2.0 },
        Edge { pair: Pair { source: 2, target: 3 }, provider: 1, cost: 1.0 },
    ]);
}
