#[allow(unused_imports)]
use num_traits::Float;
#[cfg(not(feature = "std"))]
use alloc::{collections::{BTreeMap, BTreeSet}, vec, vec::Vec};
#[cfg(feature = "std")]
use std::collections::{BTreeMap, BTreeSet};
use core::cmp::Ordering;

/// Internal algo representation of source & target vertices.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub (crate) struct Pair {
    pub (crate) source: usize,
    pub (crate) target: usize,
}

/// Internal algo representation of and edge comprising a pair, provider, cost.
#[derive(Copy, Clone, Debug)]
pub (crate) struct Edge {
    pub (crate) pair: Pair,
    pub (crate) provider: usize,
    pub (crate) cost: f64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.pair == other.pair && self.provider == other.provider && self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pair.cmp(&other.pair)
            .then_with(|| self.provider.cmp(&other.provider))
            .then_with(|| self.cost.partial_cmp(&other.cost).unwrap())
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub (crate) struct Path {
    pub (crate) total_cost: f64,
    pub (crate) edges: Vec<Edge>,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost && self.edges == other.edges
    }
}

impl Eq for Path {}

impl Path {
    pub (crate) fn add(&mut self, edge: &Edge) {
        self.total_cost += edge.cost;
        self.edges.push(*edge);
    }
}

#[derive(Debug)]
pub enum PathCalculationError {
    NegativeCyclesError,
}

/// Gets longest paths, as per: https://www.coursera.org/lecture/algorithms-on-graphs/currency-exchange-reduction-to-shortest-paths-cw8Tm
/// - switch weights to log2(w) to allow for shortest_paths() addition of weights
/// - negate log2(w) in order to reuse shortest_paths()
/// 
/// Formula, given: x*y = 2^(log2(x) + log2(y))
/// maximizing x*y is equivalent to maximizing log2(x) + log2(y)
/// ie. can convert weights x => log2(x), y => log2(y)
/// Negate the weight for compatibility with shortest_path()
pub (crate) fn longest_paths_mult(edges: &[Edge]) -> Result<BTreeMap<Pair, Path>, PathCalculationError> {
    let edges = unique_cheapest_edges(edges, Ordering::Greater);
    // record the original weights
    let weight_map: BTreeMap<(Pair, usize), f64> = edges.iter().map(|e|((e.pair, e.provider), e.cost)).collect();
    // map weights x => log2(x)
    let edges_with_log_weights: Vec<Edge> = edges.iter().map(|e|Edge{cost: -e.cost.log2(), ..*e}).collect() ;

    // run longest path algo
    let res = shortest_paths(&edges_with_log_weights);

    // map weights back to x, recalculate total_cost
    res.map(|res_map|{
        res_map.iter().map(|(pair, path)|{
            let edges_iter = path.edges.iter().map(|e|{
                Edge{cost: weight_map[&(e.pair, e.provider)], ..*e}
            });
            let total_cost = edges_iter.clone().fold(1.0, |acc, e| acc * e.cost);
            let path = Path{total_cost, edges: edges_iter.collect::<Vec<_>>()};
            (*pair, path)
        }).collect::<BTreeMap<Pair, Path>>()
    })
}

pub (crate) fn shortest_paths(edges: &[Edge]) -> Result<BTreeMap<Pair, Path>, PathCalculationError> {
    floyd_warshall_shortest_paths(&unique_cheapest_edges(edges, Ordering::Less))
}

// Floyd-Warshall shortest path algorithm.
// Utilizes simple data structures with range of usize
fn floyd_warshall_shortest_paths(edges: &[Edge]) -> Result<BTreeMap<Pair, Path>, PathCalculationError> {
    let mut vertices: BTreeSet<usize> = BTreeSet::new();
    let mut edges_by_pair: BTreeMap<Pair, Edge> = BTreeMap::new();
    let mut paths_by_pair: BTreeMap<Pair, Path> = BTreeMap::new();

    for e in edges.iter() {
        vertices.insert(e.pair.source);
        vertices.insert(e.pair.target);
        edges_by_pair.insert(Pair{source: e.pair.source, target: e.pair.target}, *e);
        paths_by_pair.entry(Pair{source: e.pair.source, target: e.pair.target}).or_insert(Path{total_cost: 0.0, edges: vec![]}).add(e);
    }

    let mut matrix: BTreeMap<Pair, Path> = BTreeMap::new();
    // initial setup based on edges
    for v in vertices.iter() {
        matrix.insert(Pair{source: *v, target: *v}, Path{total_cost: 0.0, edges: vec![]});
    }
    for e in edges.iter() {
        matrix.insert(Pair{source: e.pair.source, target: e.pair.target}, Path{total_cost: e.cost, edges: vec![*e]});
    }

    // recalculate the matrix as per: https://youtu.be/oNI0rf2P9gE?t=817
    // A[i,j] = min(A[i,j], A[i,k] + A[k,j])
    for k in vertices.iter() {
        for i in vertices.iter() {
            for j in vertices.iter() {
                let ij_cost = match matrix.get(&Pair{source: *i, target: *j}) {
                    Some(ij) => ij.total_cost,
                    None           => f64::MAX  // suggests infinite cost
                };
                let (ik_cost, ik_edges) = match matrix.get(&Pair{source: *i, target: *k}) {
                    Some(ik) => (ik.total_cost, ik.edges.clone()),
                    None           => (f64::MAX, vec![])  // suggests infinite cost
                };
                let (kj_cost, kj_edges) = match matrix.get(&Pair{source: *k, target: *j}) {
                    Some(kj) => (kj.total_cost, kj.edges.clone()),
                    None           => (f64::MAX, vec![])  // suggests infinite cost
                };

                if ik_cost + kj_cost != f64::MAX && ij_cost > ik_cost + kj_cost {
                    let mut new_ij_edges = ik_edges;
                    new_ij_edges.extend(kj_edges);
                    matrix.insert(Pair{source: *i, target: *j},Path{total_cost: ik_cost + kj_cost, edges: new_ij_edges});
                }
            }
        }
    }

    // check for negative cycles
    for i in vertices {
        let contains_negative_cycle = matrix.get(&Pair{source: i, target: i}).unwrap().total_cost < 0.0;
        if contains_negative_cycle {
            return Err(PathCalculationError::NegativeCyclesError)
        }
    }

    Ok(matrix)
}

pub (crate) fn unique_cheapest_edges(edges: &[Edge], ordering: Ordering) -> Vec<Edge> {
    let mut edges_by_pair: BTreeMap<(usize, usize), Edge> = BTreeMap::new();
    for e in edges.iter() {
        edges_by_pair.entry((e.pair.source, e.pair.target))
            .and_modify(|old_costed| if old_costed.cost.partial_cmp(&e.cost).unwrap() == ordering { *old_costed = *e })
            .or_insert_with(|| *e);
    }
    edges_by_pair.values().cloned().collect::<Vec<_>>()
}