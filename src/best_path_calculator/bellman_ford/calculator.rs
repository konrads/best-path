#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use alloc::{
    collections::BTreeMap,
    vec, vec::Vec,
};
use super::algo;
use super::super::PositionIndexer;
use crate::types::*;
use crate::*;
#[cfg(feature = "std")]
use std::collections::BTreeMap;
use core::cmp::Ordering;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::Float;

pub use super::super::SCALE;

pub struct BellmanFordCalculator {}

impl<C: Currency, A: Amount, P: Provider> SingleSourceBestPathCalculator<C, A, P>
    for BellmanFordCalculator
{
    /// Calculates best paths from a single source currency using Bellman-Ford.
    /// O(VE) vs Floyd-Warshall's O(V³) — efficient when only one source is needed.
    ///
    /// Returns a `PricePathGraph` keyed by `Pair { source, target }` for every reachable target,
    /// including the self-loop `source → source` with `total_cost = SCALE` and empty steps.
    fn calc_best_paths_from(
        source: &C,
        pairs_and_prices: &[(ProviderPair<C, P>, A)],
    ) -> Result<PricePathGraph<C, A, P>, CalculatorError> {
        if pairs_and_prices.is_empty() {
            return Ok(BTreeMap::new());
        }

        let currency_indexer = PositionIndexer::new(
            pairs_and_prices.iter().flat_map(|(ProviderPair { pair: Pair { source, target }, .. }, ..)| {
                [source, target].into_iter()
            }),
        );
        let provider_indexer = PositionIndexer::new(
            pairs_and_prices.iter().map(|(ProviderPair { provider, .. }, ..)| provider),
        );

        let source_idx = match currency_indexer.try_by_val(source) {
            Some(idx) => idx,
            None => return Ok(BTreeMap::new()),
        };

        // Build edges O(E)
        let graph = pairs_and_prices.iter().map(|(pp, cost)| {
            Ok(algo::Edge {
                pair:     algo::Pair {
                    source: currency_indexer.by_val(&pp.pair.source),
                    target: currency_indexer.by_val(&pp.pair.target),
                },
                provider: provider_indexer.by_val(&pp.provider),
                cost:     TryInto::<u128>::try_into(*cost)
                    .map_err(|_| CalculatorError::ConversionError)? as f64 / SCALE,
            })
        }).collect::<Result<Vec<algo::Edge>, CalculatorError>>()?;

        // Deduplicate and apply log transform (same technique as Floyd-Warshall)
        let deduped = algo::best_edge_per_pair(&graph, Ordering::Greater);
        let weight_map: BTreeMap<(algo::Pair, usize), f64> = deduped.iter()
            .map(|e| ((e.pair, e.provider), e.cost))
            .collect();
        let log_edges: Vec<algo::Edge> = deduped.iter()
            .map(|e| algo::Edge { cost: -e.cost.log2(), ..*e })
            .collect();

        let res = algo::bellman_ford(&log_edges, source_idx)?;

        let source_currency = currency_indexer.by_ind(source_idx);
        let res_map = res.into_iter()
            .map(|(target_idx, algo::Path { edges, .. })| {
                let pair = Pair {
                    source: source_currency.clone(),
                    target: currency_indexer.by_ind(target_idx),
                };
                let steps: Vec<PathStep<C, A, P>> = edges.into_iter().map(|algo::Edge { pair: algo::Pair { source, target }, provider, .. }| {
                    PathStep {
                        pair: Pair {
                            source: currency_indexer.by_ind(source),
                            target: currency_indexer.by_ind(target),
                        },
                        provider: provider_indexer.by_ind(provider),
                        cost: ((weight_map[&(algo::Pair { source, target }, provider)] * SCALE) as u128)
                            .try_into().ok().unwrap(),
                    }
                }).collect();
                // total_cost = product of original exchange rates (same as FW), not log-space distance
                let total_cost_f64 = steps.iter()
                    .map(|s| TryInto::<u128>::try_into(s.cost).unwrap_or(0) as f64 / SCALE)
                    .fold(1.0_f64, |acc, r| acc * r);
                let path = PricePath {
                    total_cost: ((total_cost_f64 * SCALE) as u128).try_into().ok().unwrap(),
                    steps,
                };
                (pair, path)
            })
            .collect::<BTreeMap<_, _>>();

        Ok(res_map)
    }
}
