pub mod noop_calculator;
pub mod floyd_warshall;
pub mod bellman_ford;
pub(crate) mod algo_types;

pub const SCALE: f64 = 1_000_000_000_000.0;

#[cfg(not(feature = "std"))]
use alloc::{collections::{BTreeMap, BTreeSet}, vec::Vec};
#[cfg(feature = "std")]
use std::collections::{BTreeMap, BTreeSet};

/// Utility for indexing a set of values by sorted position.
/// Provides O(log n) lookup by value and O(1) lookup by index.
pub(crate) struct PositionIndexer<'a, T> {
    map: BTreeMap<&'a T, usize>,
    vec: Vec<&'a T>,
}

impl<'a, T: Ord + Clone> PositionIndexer<'a, T> {
    pub(crate) fn new(set: impl Iterator<Item = &'a T>) -> Self {
        let vec = set.collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        let map = vec.iter().enumerate().map(|(i, x)| (*x, i)).collect::<BTreeMap<_, _>>();
        Self { map, vec }
    }

    pub(crate) fn by_val(&self, val: &'a T) -> usize {
        *self.map.get(val).unwrap()
    }

    /// Like by_val but accepts a reference of any lifetime — used when the caller
    /// holds a value that isn't tied to the indexer's source slice (e.g. a `source`
    /// parameter passed from outside).
    pub(crate) fn try_by_val(&self, val: &T) -> Option<usize> {
        self.map.get(val).copied()
    }

    pub(crate) fn by_ind(&self, ind: usize) -> T {
        self.vec.get(ind).map(|&x| x.clone()).unwrap()
    }
}
