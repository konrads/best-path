#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use alloc::{string::String, vec, vec::Vec};
use crate::types::*;
use crate::{AllPairsBestPathCalculator, SingleSourceBestPathCalculator};
use super::calculator::{BellmanFordCalculator, SCALE};
use crate::best_path_calculator::floyd_warshall::calculator::FloydWarshallCalculator;

fn real_life_graph() -> Vec<(ProviderPair<String, String>, u128)> {
    const P: &str = "P";
    vec![
        ("BTC",  "USDT", 5997.42),
        ("USDT", "BTC",  0.00002777),
        ("ETH",  "USDT", 2384.99),
        ("USDT", "ETH",  0.0004192),
        ("BNB",  "USDT", 364.19),
        ("USDT", "BNB",  0.002745),
        ("DOT",  "USDT", 17.43),
        ("USDT", "DOT",  0.05737),
        ("BTC",  "ETH",  15.09),
        ("ETH",  "BTC",  0.06626),
        ("ETH",  "BNB",  6.548),
        ("BNB",  "ETH",  0.1527),
    ].into_iter().map(|(src, tgt, rate)| (
        ProviderPair {
            pair: Pair { source: src.to_owned(), target: tgt.to_owned() },
            provider: P.to_owned(),
        },
        (rate * SCALE) as u128,
    )).collect()
}

/// BF from source X must return the same paths as FW filtered to rows where source == X.
#[test]
fn test_parity_with_floyd_warshall() {
    let graph = real_life_graph();
    let source = "BNB".to_owned();

    let fw = FloydWarshallCalculator::calc_best_paths(&graph).unwrap();
    let bf = BellmanFordCalculator::calc_best_paths_from(&source, &graph).unwrap();

    // every BF entry must match the corresponding FW entry
    for (pair, bf_path) in &bf {
        let fw_path = fw.get(pair).unwrap_or_else(|| panic!("FW missing {pair:?}"));
        assert_eq!(
            bf_path.total_cost, fw_path.total_cost,
            "total_cost mismatch for {pair:?}"
        );
        assert_eq!(
            bf_path.steps.len(), fw_path.steps.len(),
            "step count mismatch for {pair:?}"
        );
        for (i, (bs, fs)) in bf_path.steps.iter().zip(fw_path.steps.iter()).enumerate() {
            assert_eq!(bs.pair, fs.pair, "step {i} pair mismatch for {pair:?}");
            assert_eq!(bs.provider, fs.provider, "step {i} provider mismatch for {pair:?}");
            assert_eq!(bs.cost, fs.cost, "step {i} cost mismatch for {pair:?}");
        }
    }

    // BF must cover every FW row that has this source
    let fw_from_source: Vec<_> = fw.keys().filter(|p| p.source == source).collect();
    assert_eq!(
        bf.len(), fw_from_source.len(),
        "BF returned {} paths, FW has {} paths from {source}",
        bf.len(), fw_from_source.len()
    );
}

/// Self-loop: source→source must have total_cost = SCALE (exchange rate 1.0), empty steps.
#[test]
fn test_self_loop_cost() {
    let graph = real_life_graph();
    let source = "ETH".to_owned();
    let res = BellmanFordCalculator::calc_best_paths_from(&source, &graph).unwrap();
    let self_path = res.get(&Pair { source: source.clone(), target: source.clone() }).unwrap();
    assert_eq!(self_path.total_cost, 1_000_000_000_000_u128);
    assert!(self_path.steps.is_empty());
}

/// Multi-hop: BNB→BTC goes via ETH; total_cost must equal product of step costs.
#[test]
fn test_multi_hop_total_cost() {
    let graph = real_life_graph();
    let source = "BNB".to_owned();
    let res = BellmanFordCalculator::calc_best_paths_from(&source, &graph).unwrap();

    let path = res.get(&Pair { source: "BNB".to_owned(), target: "BTC".to_owned() }).unwrap();
    assert_eq!(path.steps.len(), 2, "BNB→BTC should go via ETH");

    // total_cost must equal product of step costs (same invariant as FW)
    let product: u128 = path.steps.iter()
        .map(|s| s.cost as f64 / SCALE)
        .fold(1.0_f64, |acc, r| acc * r)
        .mul_add(SCALE, 0.0) as u128;
    assert_eq!(path.total_cost, product);
}

/// Unknown source returns empty map (not an error).
#[test]
fn test_unknown_source_returns_empty() {
    let graph = real_life_graph();
    let res = BellmanFordCalculator::calc_best_paths_from(&"XYZ".to_owned(), &graph).unwrap();
    assert!(res.is_empty());
}
