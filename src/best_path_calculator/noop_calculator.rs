#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeMap;
use crate::types::*;
use crate::BestPathCalculator;

pub struct NoBestPathCalculator {}
impl<C: Currency, A: Amount, P: Provider> BestPathCalculator<C, A, P> for NoBestPathCalculator {
	fn calc_best_paths(pairs_and_prices: &[(ProviderPair<C, P>, A)]) -> Result<BTreeMap<Pair<C>, PricePath<C, A, P>>, CalculatorError> {
		Ok(pairs_and_prices.iter().cloned().map(|(pp, price)| (Pair{source: pp.pair.source, target: pp.pair.target}, PricePath{total_cost: price, steps: vec![]})).collect())
	}
}