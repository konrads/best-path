#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "scale")]
use codec::{Decode, Encode};
#[cfg(feature = "scale")]
use scale_info::TypeInfo;

/// Currency representation, eg. "BTC".
pub trait Currency: Ord + Clone + AsRef<[u8]> {}
impl<T: Ord + Clone + AsRef<[u8]>> Currency for T {}

/// Numeric amount, must be representable by u128.
pub trait Amount: Clone + Copy + TryInto<u128> + TryFrom<u128> {}
impl<T: Clone + Copy + TryInto<u128> + TryFrom<u128>> Amount for T {}

/// Oracle providing currency pair pricing.
pub trait Provider: Ord + Clone {}
impl<T: Ord + Clone> Provider for T {}

/// Currency  pair, as provided by Provider.
#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct Pair<C: Currency> {
    pub source: C,
    pub target: C,
}

/// Currency pair per provider.
#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct ProviderPair<C: Currency, P: Provider> {
    pub pair: Pair<C>,
    pub provider: P,
}

/// Path for every ProviderPair. Consists of `steps` and overall cost
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub struct PricePath<C: Currency, A: Amount, P: Provider> {
    pub total_cost: A,
    pub steps: Vec<PathStep<C, A, P>>,
}

/// Conversion cost of source/target currency, per provider. Can be used as a building block for longer paths when there's no direct route.
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
