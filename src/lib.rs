#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use sp_std::result::{Result, Result::*};

#[cfg(not(feature = "std"))]
use sp_std::collections::btree_map::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeMap;

pub mod best_path_calculator;
pub mod types;
use types::*;

pub trait BestPathCalculator<C: Currency, A: Amount> {
    fn calc_best_paths(
        pairs_and_prices: &[(ProviderPair<C>, A)],
    ) -> Result<BTreeMap<Pair<C>, PricePath<C, A>>, CalculatorError>;
}
