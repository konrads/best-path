#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap, vec, vec::Vec};
#[cfg(feature = "std")]
use std::collections::BTreeMap;
pub(crate) use super::super::algo_types::{Edge, Pair, Path, PathCalculationError, best_edge_per_pair};

/// Bellman-Ford single-source shortest paths.
///
/// Returns `target_vertex → Path` for every vertex reachable from `source`.
/// The source itself maps to an empty path with zero cost.
/// Vertices unreachable from `source` are absent from the result.
pub(crate) fn bellman_ford(
    edges: &[Edge],
    source: usize,
) -> Result<BTreeMap<usize, Path>, PathCalculationError> {
    if edges.is_empty() {
        return Ok(BTreeMap::new());
    }

    let n = edges.iter()
        .flat_map(|e| [e.pair.source, e.pair.target])
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);

    if source >= n {
        return Ok(BTreeMap::new());
    }

    let mut vertex_exists = vec![false; n];
    for &e in edges {
        vertex_exists[e.pair.source] = true;
        vertex_exists[e.pair.target] = true;
    }

    // dist[v]      = best known cost source→v
    // prev_edge[v] = incoming edge on the best known path to v
    let mut dist      = vec![f64::INFINITY; n];
    let mut prev_edge: Vec<Option<Edge>> = vec![None; n];
    dist[source] = 0.0;

    // Relax all edges V−1 times; early-exit if a full pass makes no progress
    for _ in 0..n.saturating_sub(1) {
        let mut updated = false;
        for &e in edges {
            let d = dist[e.pair.source];
            if d.is_infinite() { continue; }
            let new_d = d + e.cost;
            if new_d < dist[e.pair.target] {
                dist[e.pair.target] = new_d;
                prev_edge[e.pair.target] = Some(e);
                updated = true;
            }
        }
        if !updated { break; }
    }

    // Vth pass: any edge that can still be relaxed is part of a negative cycle
    for &e in edges {
        if !dist[e.pair.source].is_infinite() && dist[e.pair.source] + e.cost < dist[e.pair.target] {
            return Err(PathCalculationError::NegativeCyclesError);
        }
    }

    // Reconstruct paths by following the predecessor chain backwards, then reversing
    let mut result: BTreeMap<usize, Path> = BTreeMap::new();
    result.insert(source, Path { total_cost: 0.0, edges: vec![] });

    for target in 0..n {
        if target == source || dist[target].is_infinite() || !vertex_exists[target] { continue; }

        let mut path_edges = vec![];
        let mut cur = target;
        while cur != source {
            match prev_edge[cur] {
                Some(e) => {
                    path_edges.push(e);
                    cur = e.pair.source;
                }
                None => break,
            }
        }
        path_edges.reverse();

        result.insert(target, Path { total_cost: dist[target], edges: path_edges });
    }

    Ok(result)
}
