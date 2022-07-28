#[cfg(not(feature = "std"))]
use alloc::vec;
use crate::types::*;
use crate::*;

pub struct NoBestPathCalculator {}
impl<C: Currency, A: Amount, P: Provider> BestPathCalculator<C, A, P> for NoBestPathCalculator {
	fn calc_best_paths(pairs_and_prices: &[(ProviderPair<C, P>, A)]) -> Result<PricePathGraph<C, A, P>, CalculatorError> {
		Ok(pairs_and_prices.iter().cloned().map(|(pp, price)| (Pair{source: pp.pair.source, target: pp.pair.target}, PricePath{total_cost: price, steps: vec![]})).collect())
	}
}