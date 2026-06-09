#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap, vec::Vec};
#[cfg(feature = "std")]
use std::collections::BTreeMap;
use core::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Pair {
    pub(crate) source: usize,
    pub(crate) target: usize,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Edge {
    pub(crate) pair: Pair,
    pub(crate) provider: usize,
    pub(crate) cost: f64,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Path {
    pub(crate) total_cost: f64,
    pub(crate) edges: Vec<Edge>,
}

#[derive(Debug)]
pub(crate) enum PathCalculationError {
    NegativeCyclesError,
}

impl From<PathCalculationError> for crate::types::CalculatorError {
    fn from(err: PathCalculationError) -> Self {
        match err {
            PathCalculationError::NegativeCyclesError => crate::types::CalculatorError::NegativeCyclesError,
        }
    }
}

/// Returns one edge per (source, target) pair, keeping whichever edge wins under `ordering`.
///
/// The winner is the edge whose cost, when compared to the incumbent via `partial_cmp`,
/// returns `ordering`. Concretely:
/// - `Ordering::Greater` → keeps the **minimum** cost edge (old > new ⇒ replace)
/// - `Ordering::Less`    → keeps the **maximum** cost edge (old < new ⇒ replace)
pub(crate) fn best_edge_per_pair(edges: &[Edge], ordering: Ordering) -> Vec<Edge> {
    let mut by_pair: BTreeMap<(usize, usize), Edge> = BTreeMap::new();
    for e in edges {
        by_pair.entry((e.pair.source, e.pair.target))
            .and_modify(|old| if old.cost.partial_cmp(&e.cost).unwrap() == ordering { *old = *e })
            .or_insert_with(|| *e);
    }
    by_pair.values().cloned().collect()
}
