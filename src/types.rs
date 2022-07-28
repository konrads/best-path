#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "scale")]
use codec::{Decode, Encode};
#[cfg(feature = "scale")]
use scale_info::TypeInfo;
#[cfg(not(feature = "std"))]
use sp_std::str;
#[cfg(feature = "std")]
use std::str;

pub trait Conversions {
    fn to_str(&self) -> &str;
    fn from_vecu8(vec: Vec<u8>) -> Self;
}

impl Conversions for Vec<u8> {
    fn to_str(&self) -> &str {
        str::from_utf8(self).ok().unwrap()
    }
    fn from_vecu8(vec: Vec<u8>) -> Self {
        vec
    }
}

pub trait Currency: Ord + Conversions + Clone + AsRef<[u8]> {}
impl<T: Ord + Conversions + Clone + AsRef<[u8]>> Currency for T {}

#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
#[allow(clippy::upper_case_acronyms)]
pub enum Provider {
    CRYPTOCOMPARE,
}
pub trait Amount: Clone + Copy + TryInto<u128> + TryFrom<u128> {}
impl<T: Clone + Copy + TryInto<u128> + TryFrom<u128>> Amount for T {}

/// Per provider, source and target currency. Represents price points from each provider
#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct Pair<C: Currency> {
    pub source: C,
    pub target: C,
}

/// Per provider, source and target currency. Represents price points from each provider
#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct ProviderPair<C: Currency> {
    pub pair: Pair<C>,
    pub provider: Provider,
}

/// Path for every ProviderPair. Consists of `hops` and overall cost
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct PricePath<C: Currency, A: Amount> {
    pub total_cost: A,
    pub steps: Vec<PathStep<C, A>>,
}

/// A `hop` between different currencies, via a provider.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct PathStep<C: Currency, A: Amount> {
    pub pair: Pair<C>,
    pub provider: Provider,
    pub cost: A,
}

#[derive(Debug)]
pub enum CalculatorError {
    NegativeCyclesError,
    ConversionError,
}
