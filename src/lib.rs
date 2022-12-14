#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeMap;

mod best_path_calculator;
mod types;
use types::*;

pub type PricePathGraph<C, A, P> = BTreeMap<Pair<C>, PricePath<C, A, P>>;

/// Interface for calculating best PricePathGraph, which consists of PricePath chained in most trade efficient manner.
pub trait BestPathCalculator<C: Currency, A: Amount, P: Provider> {
    fn calc_best_paths(pairs_and_prices: &[(ProviderPair<C, P>, A)]) -> Result<PricePathGraph<C, A, P>, CalculatorError>;
}

pub mod prelude {
    pub use super::best_path_calculator::*;
    pub use super::types::*;
    pub use super::*;
}