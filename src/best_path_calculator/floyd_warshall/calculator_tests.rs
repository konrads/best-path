#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use alloc::{
    borrow::ToOwned,
    string::String,
    vec, vec::Vec,
};
use crate::types::*;
use crate::BestPathCalculator;
use super::calculator::*;

const MOCK_PROVIDER: &str = "MOCK_PROVIDER";

#[test]
fn test_real_life_graph() {
    /*
    Test prices generated via:
    curl https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USDT  # {"USDT":35997.42}
    curl https://min-api.cryptocompare.com/data/price?fsym=USDT&tsyms=BTC  # {"BTC":0.00002778}
    curl https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USDT  # {"USDT":2384.99}
    curl https://min-api.cryptocompare.com/data/price?fsym=USDT&tsyms=ETH  # {"ETH":0.0004194}
    curl https://min-api.cryptocompare.com/data/price?fsym=BNB&tsyms=USDT  # {"USDT":364.19}
    curl https://min-api.cryptocompare.com/data/price?fsym=USDT&tsyms=BNB  # {"BNB":0.002746}
    curl https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USDT  # {"USDT":17.43}
    curl https://min-api.cryptocompare.com/data/price?fsym=USDT&tsyms=DOT  # {"DOT":0.05737}
    curl https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=ETH   # {"ETH":15.09}
    curl https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=BTC   # {"BTC":0.06627}
    curl https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=BNB   # {"BNB":6.548}
    curl https://min-api.cryptocompare.com/data/price?fsym=BNB&tsyms=ETH   # {"ETH":0.1527}
    */
    let in_graph = vec![
        ("BTC".to_owned(),  "USDT".to_owned(), MOCK_PROVIDER, 5997.42),
        ("USDT".to_owned(), "BTC".to_owned(),  MOCK_PROVIDER, 0.00002777),
        ("ETH".to_owned(),  "USDT".to_owned(), MOCK_PROVIDER, 2384.99),
        ("USDT".to_owned(), "ETH".to_owned(),  MOCK_PROVIDER, 0.0004192),
        ("BNB".to_owned(),  "USDT".to_owned(), MOCK_PROVIDER, 364.19),
        ("USDT".to_owned(), "BNB".to_owned(),  MOCK_PROVIDER, 0.002745),
        ("DOT".to_owned(),  "USDT".to_owned(), MOCK_PROVIDER, 17.43),
        ("USDT".to_owned(), "DOT".to_owned(),  MOCK_PROVIDER, 0.05737),
        ("BTC".to_owned(),  "ETH".to_owned(),  MOCK_PROVIDER, 15.09),
        ("ETH".to_owned(),  "BTC".to_owned(),  MOCK_PROVIDER, 0.06626),
        ("ETH".to_owned(),  "BNB".to_owned(),  MOCK_PROVIDER, 6.548),
        ("BNB".to_owned(),  "ETH".to_owned(),  MOCK_PROVIDER, 0.1527),
    ].into_iter().map(|(source, target, provider, cost)| (ProviderPair{pair: Pair{source: source.as_str().as_bytes().to_vec(), target: target.as_str().as_bytes().to_vec()}, provider}, (cost * PRECISION) as u128)).collect::<Vec<_>>();
    let res_out = FloydWarshallCalculator::calc_best_paths(&in_graph).unwrap().into_iter().collect::<Vec<(_, _)>>()
        .into_iter().map(|(p, pp)|(
            String::from_utf8(p.source).unwrap(),
            String::from_utf8(p.target).unwrap(),
            pp.total_cost as f64 / PRECISION,
            pp.steps.into_iter().map(|PathStep{pair: Pair{source, target}, provider, cost}| (
                String::from_utf8(source).unwrap(),
                String::from_utf8(target).unwrap(),
                provider,
                cost as f64 / PRECISION,
            )).collect::<Vec<(String, String, &str, f64)>>())
        ).collect::<Vec<(String, String, f64, Vec<(String, String, &str, f64)>)>>();
    assert_eq!(
        vec![
            ("BNB".to_owned(), "BNB".to_owned(), 1.0,                vec![]),
            ("BNB".to_owned(), "BTC".to_owned(), 0.010117902,        vec![("BNB".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 0.1527), ("ETH".to_owned(), "BTC".to_owned(), MOCK_PROVIDER, 0.06626)]),
            ("BNB".to_owned(), "DOT".to_owned(), 20.8935803,         vec![("BNB".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 364.19), ("USDT".to_owned(), "DOT".to_owned(), MOCK_PROVIDER, 0.05737)]),
            ("BNB".to_owned(), "ETH".to_owned(), 0.1527,             vec![("BNB".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 0.1527)]),
            ("BNB".to_owned(), "USDT".to_owned(), 364.19,            vec![("BNB".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 364.19)]),
            ("BTC".to_owned(), "BNB".to_owned(), 98.80932,           vec![("BTC".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 15.09), ("ETH".to_owned(), "BNB".to_owned(), MOCK_PROVIDER, 6.548)]),
            ("BTC".to_owned(), "BTC".to_owned(), 1.0,                vec![]),
            ("BTC".to_owned(), "DOT".to_owned(), 2064.717563366999,  vec![("BTC".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 15.09), ("ETH".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 2384.99), ("USDT".to_owned(), "DOT".to_owned(), MOCK_PROVIDER, 0.05737)]),
            ("BTC".to_owned(), "ETH".to_owned(), 15.09,              vec![("BTC".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 15.09)]),
            ("BTC".to_owned(), "USDT".to_owned(), 35989.49909999999, vec![("BTC".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 15.09), ("ETH".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 2384.99)]),
            ("DOT".to_owned(), "BNB".to_owned(), 0.04784535,         vec![("DOT".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 17.43), ("USDT".to_owned(), "BNB".to_owned(), MOCK_PROVIDER, 0.002745)]),
            ("DOT".to_owned(), "BTC".to_owned(), 0.000484139026,     vec![("DOT".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 17.43), ("USDT".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 0.0004192), ("ETH".to_owned(), "BTC".to_owned(), MOCK_PROVIDER, 0.06626)]),
            ("DOT".to_owned(), "DOT".to_owned(), 1.0,                vec![]),
            ("DOT".to_owned(), "ETH".to_owned(), 0.007306656,        vec![("DOT".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 17.43), ("USDT".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 0.0004192)]),
            ("DOT".to_owned(), "USDT".to_owned(), 17.43,             vec![("DOT".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 17.43)]),
            ("ETH".to_owned(), "BNB".to_owned(), 6.548,              vec![("ETH".to_owned(), "BNB".to_owned(), MOCK_PROVIDER, 6.548)]),
            ("ETH".to_owned(), "BTC".to_owned(), 0.06626,            vec![("ETH".to_owned(), "BTC".to_owned(), MOCK_PROVIDER, 0.06626)]),
            ("ETH".to_owned(), "DOT".to_owned(), 136.826876299999,   vec![("ETH".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 2384.99), ("USDT".to_owned(), "DOT".to_owned(), MOCK_PROVIDER, 0.05737)]),
            ("ETH".to_owned(), "ETH".to_owned(), 1.0,                vec![]),
            ("ETH".to_owned(), "USDT".to_owned(), 2384.99,           vec![("ETH".to_owned(), "USDT".to_owned(), MOCK_PROVIDER, 2384.99)]),
            ("USDT".to_owned(), "BNB".to_owned(), 0.002745,          vec![("USDT".to_owned(), "BNB".to_owned(), MOCK_PROVIDER, 0.002745)]),
            ("USDT".to_owned(), "BTC".to_owned(), 2.7776192e-5,      vec![("USDT".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 0.0004192), ("ETH".to_owned(), "BTC".to_owned(), MOCK_PROVIDER, 0.06626)]),
            ("USDT".to_owned(), "DOT".to_owned(), 0.05737,           vec![("USDT".to_owned(), "DOT".to_owned(), MOCK_PROVIDER, 0.05737)]),
            ("USDT".to_owned(), "ETH".to_owned(), 0.0004192,         vec![("USDT".to_owned(), "ETH".to_owned(), MOCK_PROVIDER, 0.0004192)]),
            ("USDT".to_owned(), "USDT".to_owned(), 1.0,              vec![])
        ],
        res_out);
}