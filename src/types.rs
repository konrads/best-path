#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "scale")]
use codec::{Decode, Encode};
#[cfg(feature = "scale")]
use scale_info::TypeInfo;

pub trait Currency: Ord + Clone + AsRef<[u8]> {}
impl<T: Ord + Clone + AsRef<[u8]>> Currency for T {}

pub trait Amount: Clone + Copy + TryInto<u128> + TryFrom<u128> {}
impl<T: Clone + Copy + TryInto<u128> + TryFrom<u128>> Amount for T {}

pub trait Provider: Ord + Clone {}
impl<T: Ord + Clone> Provider for T {}

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
pub struct ProviderPair<C: Currency, P: Provider> {
    pub pair: Pair<C>,
    pub provider: P,
}

/// Path for every ProviderPair. Consists of `hops` and overall cost
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct PricePath<C: Currency, A: Amount, P: Provider> {
    pub total_cost: A,
    pub steps: Vec<PathStep<C, A, P>>,
}

/// A `hop` between different currencies, via a provider.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct PathStep<C: Currency, A: Amount, P: Provider> {
    pub pair: Pair<C>,
    pub provider: P,
    pub cost: A,
}

#[derive(Debug)]
pub enum CalculatorError {
    NegativeCyclesError,
    ConversionError,
}
