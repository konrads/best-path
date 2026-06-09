#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap, vec, vec::Vec};
#[cfg(feature = "std")]
use std::collections::BTreeMap;
use core::cmp::Ordering;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::Float;
pub(crate) use super::super::algo_types::{Edge, Pair, Path, PathCalculationError, best_edge_per_pair};

/// Gets longest paths, as per: https://www.coursera.org/lecture/algorithms-on-graphs/currency-exchange-reduction-to-shortest-paths-cw8Tm
/// - switch weights to log2(w) to allow for shortest_paths() addition of weights
/// - negate log2(w) in order to reuse shortest_paths()
///
/// Formula, given: x*y = 2^(log2(x) + log2(y))
/// maximizing x*y is equivalent to maximizing log2(x) + log2(y)
/// ie. can convert weights x => log2(x), y => log2(y)
/// Negate the weight for compatibility with shortest_path()
pub (crate) fn longest_paths_by_product(edges: &[Edge]) -> Result<BTreeMap<Pair, Path>, PathCalculationError> {
    let deduped = best_edge_per_pair(edges, Ordering::Greater);
    // record the original weights before log-transforming
    let weight_map: BTreeMap<(Pair, usize), f64> = deduped.iter().map(|e|((e.pair, e.provider), e.cost)).collect();
    // map weights x => -log2(x) for use with shortest-path core
    let log_edges: Vec<Edge> = deduped.iter().map(|e| Edge { cost: -e.cost.log2(), ..*e }).collect();

    // call core directly — deduplication already done above, no second pass
    let res = floyd_warshall_core(&log_edges)?;

    // map costs back to original scale
    Ok(res.into_iter().map(|(pair, path)| {
        let edges_iter = path.edges.iter().map(|e| {
            Edge { cost: weight_map[&(e.pair, e.provider)], ..*e }
        });
        let total_cost = edges_iter.clone().fold(1.0, |acc, e| acc * e.cost);
        (pair, Path { total_cost, edges: edges_iter.collect::<Vec<_>>() })
    }).collect::<BTreeMap<Pair, Path>>())
}

#[cfg(test)]
pub (crate) fn shortest_paths(edges: &[Edge]) -> Result<BTreeMap<Pair, Path>, PathCalculationError> {
    floyd_warshall_core(&best_edge_per_pair(edges, Ordering::Less))
}

// Floyd-Warshall core.
//
// Flat Vec cost/next-hop matrices, vertex IDs used directly as indices (PositionIndexer
// guarantees dense 0..n IDs — no remapping needed). Paths reconstructed from the next-hop
// table after the triple loop; no allocations inside the O(n³) iterations.
// Row constants hoisted out of the j loop; updates are branchless (cmov-friendly).
fn floyd_warshall_core(edges: &[Edge]) -> Result<BTreeMap<Pair, Path>, PathCalculationError> {
    if edges.is_empty() {
        return Ok(BTreeMap::new());
    }

    // vertex IDs are dense 0..n; derive n from the max ID present
    let n = edges.iter()
        .flat_map(|e| [e.pair.source, e.pair.target])
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);

    // cost[i*n+j]     = best known cost i→j  (INFINITY = no path)
    // next_hop[i*n+j] = first vertex after i on best path to j (usize::MAX = no path)
    // edge_for[i*n+j] = direct edge i→j, for path reconstruction
    let mut cost     = vec![f64::INFINITY; n * n];
    let mut next_hop = vec![usize::MAX;    n * n];
    let mut edge_for: Vec<Option<Edge>> = vec![None; n * n];

    // track which vertex IDs actually appear, to avoid emitting phantom self-loops
    let mut vertex_exists = vec![false; n];

    for &e in edges {
        let i = e.pair.source;
        let j = e.pair.target;
        vertex_exists[i] = true;
        vertex_exists[j] = true;
        cost[i * n + i]     = 0.0;
        cost[j * n + j]     = 0.0;
        cost[i * n + j]     = e.cost;
        next_hop[i * n + j] = j;
        edge_for[i * n + j] = Some(e);
    }

    // A[i,j] = min(A[i,j], A[i,k] + A[k,j])
    // Row constants hoisted; updates are branchless to enable cmov / auto-vectorisation.
    for k in 0..n {
        for i in 0..n {
            let cost_ik = cost[i * n + k];
            if cost_ik.is_infinite() { continue; }
            let next_hop_ik = next_hop[i * n + k];
            for j in 0..n {
                let new_cost = cost_ik + cost[k * n + j];
                let improved = new_cost < cost[i * n + j];
                cost[i * n + j]     = if improved { new_cost }      else { cost[i * n + j] };
                next_hop[i * n + j] = if improved { next_hop_ik }   else { next_hop[i * n + j] };
            }
        }
    }

    // check for negative cycles
    for i in 0..n {
        if vertex_exists[i] && cost[i * n + i] < 0.0 {
            return Err(PathCalculationError::NegativeCyclesError);
        }
    }

    // reconstruct full paths from the next-hop table — O(n²) total, done once
    let mut result: BTreeMap<Pair, Path> = BTreeMap::new();
    for i in 0..n {
        if !vertex_exists[i] { continue; }
        for j in 0..n {
            if !vertex_exists[j] { continue; }
            if cost[i * n + j].is_infinite() { continue; }
            let pair = Pair { source: i, target: j };
            let path_edges = if i == j {
                vec![]
            } else {
                let mut hops = vec![];
                let mut cur = i;
                while cur != j {
                    let nxt = next_hop[cur * n + j];
                    if nxt == usize::MAX { break; }
                    if let Some(e) = edge_for[cur * n + nxt] {
                        hops.push(e);
                    }
                    cur = nxt;
                }
                hops
            };
            result.insert(pair, Path { total_cost: cost[i * n + j], edges: path_edges });
        }
    }

    Ok(result)
}
