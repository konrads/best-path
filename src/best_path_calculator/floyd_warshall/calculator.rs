#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use alloc::{
    borrow::ToOwned,
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec, vec::Vec,
};
use super::algo;
use crate::types::*;
use crate::*;
#[cfg(feature = "std")]
use std::collections::{BTreeMap, BTreeSet};

impl From<algo::PathCalculationError> for CalculatorError {
    fn from(err: algo::PathCalculationError) -> Self {
        match err {
            algo::PathCalculationError::NegativeCyclesError => CalculatorError::NegativeCyclesError
        } 
    }
}

pub const SCALE: f64 = 1_000_000_000_000.0;

pub struct FloydWarshallCalculator {}

impl<C: Currency, A: Amount, P: Provider> BestPathCalculator<C, A, P> for FloydWarshallCalculator {

    /// Implements calculation of the best PricePathGraph, utilizing Floyd-Warshall algorithm
    /// Accepts graph represented with trait Currency, Amount, Provider and wraps these into primitive indexed internal representations. Unwraps back on exit.
    ///
    /// Typical usage below. Note all prices are in scale of 10^12, including self references, eg. cost of BNB -> BNB = 10^12
    /// ```rust
    /// # use best_path::prelude::*;
    /// # use best_path::prelude::floyd_warshall::calculator::FloydWarshallCalculator;
    /// 
    /// let in_graph = &[
    ///     (
    ///         ProviderPair { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned() },
    ///         364_190_000_000_000_u128
    ///     ),
    ///     (
    ///         ProviderPair { pair: Pair { source: "USDT".to_owned(), target: "ETH".to_owned() }, provider: "COINGECKO".to_owned() },
    ///         2_745_000_000_u128
    ///     ),
    /// ];
    /// let res_out = FloydWarshallCalculator::calc_best_paths(in_graph);
    /// let res_ref = res_out.as_ref().unwrap();
    /// assert_eq!(
    ///     &PricePath { total_cost: 999_701_550_000_u128, steps: vec![
    ///         PathStep { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned(), cost: 364_190_000_000_000_u128 },
    ///         PathStep { pair: Pair { source: "USDT".to_owned(), target: "ETH".to_owned() }, provider: "COINGECKO".to_owned(), cost: 2_745_000_000_u128 }
    ///     ] },
    ///     res_ref.get(&Pair { source: "BNB".to_owned(), target: "ETH".to_owned() }).unwrap()
    /// );
    /// ```
	fn calc_best_paths(pairs_and_prices: &[(ProviderPair<C, P>, A)]) -> Result<PricePathGraph<C, A, P>, CalculatorError> {
        // get unique and indexed currencies and providers
        let currencies = pairs_and_prices.iter().flat_map(|(ProviderPair { pair: Pair{source, target}, .. }, ..)| vec![source, target].into_iter())
            .collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        let providers = pairs_and_prices.iter().map(|(ProviderPair { provider, .. }, ..)| provider)
            .collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        let currencies_by_ind = currencies.iter().enumerate().map(|(i, &x)| (x, i)).collect::<BTreeMap<_, _>>();
        let providers_by_ind = providers.iter().enumerate().map(|(i, &x)| (x, i)).collect::<BTreeMap<_, _>>();

        // construct the graph for Floyd-Warshall lib
        let mut graph = Vec::new();
        for &c in currencies.iter() {
            for (pp, cost) in pairs_and_prices {
                if c == &pp.pair.source {
                    graph.push(algo::Edge {
                        pair:        algo::Pair{source: currencies_by_ind[&pp.pair.source], target: currencies_by_ind[&pp.pair.target]},
                        provider:    providers_by_ind[&pp.provider],
                        cost:        TryInto::<u128>::try_into(*cost).map_err(|_| CalculatorError::ConversionError)? as f64 / SCALE
                    });
                }
            }
        }

        // run Floyd-Warshall for all combinations of currencies in the graph
        let res = algo::longest_paths_mult(&graph)?;
        let res_map = res.into_iter().map(|(algo::Pair{source, target}, algo::Path{total_cost, edges})| {
            let pair = Pair{source: currencies[source].clone(), target: currencies[target].clone()};
            let total_cost_u128 = (total_cost * SCALE) as u128;
            let path = PricePath{ total_cost: total_cost_u128.try_into().ok().unwrap(), steps: edges.into_iter().map(|algo::Edge{pair: algo::Pair{source, target, ..}, provider, cost}|
                PathStep{pair: Pair{source: currencies[source].clone(), target: currencies[target].clone()}, provider: providers[provider].clone(), cost: ((cost * SCALE) as u128).try_into().ok().unwrap()}).collect()
            };
            (pair, path)
        }).collect::<BTreeMap<_, _>>();
        Ok(res_map)
	}
}
