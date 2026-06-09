#[cfg(not(feature = "std"))]
use alloc::vec;
use crate::types::*;
use crate::*;

/// No-op all-pairs calculator: returns each input pair as a single-step path with no routing.
pub struct NoOpAllPairsCalculator {}
impl<C: Currency, A: Amount, P: Provider> AllPairsBestPathCalculator<C, A, P> for NoOpAllPairsCalculator {
	fn calc_best_paths(pairs_and_prices: &[(ProviderPair<C, P>, A)]) -> Result<PricePathGraph<C, A, P>, CalculatorError> {
		Ok(pairs_and_prices.iter().cloned().map(|(pp, price)| (Pair{source: pp.pair.source, target: pp.pair.target}, PricePath{total_cost: price, steps: vec![]})).collect())
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_each_pair_as_direct_path() {
        let graph = vec![
            (ProviderPair { pair: Pair { source: "A", target: "B" }, provider: "P" }, 500_u128),
            (ProviderPair { pair: Pair { source: "B", target: "C" }, provider: "P" }, 200_u128),
        ];
        let res = NoOpAllPairsCalculator::calc_best_paths(&graph).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[&Pair { source: "A", target: "B" }], PricePath { total_cost: 500_u128, steps: vec![] });
        assert_eq!(res[&Pair { source: "B", target: "C" }], PricePath { total_cost: 200_u128, steps: vec![] });
    }

    #[test]
    fn empty_input_returns_empty_map() {
        let empty: &[(ProviderPair<&str, &str>, u128)] = &[];
        let res = NoOpAllPairsCalculator::calc_best_paths(empty).unwrap();
        assert!(res.is_empty());
    }
}