# Longest/shortest path algorithms

[![build](../../workflows/build/badge.svg)](../../actions/workflows/build.yml)

Algorithms to facilitate longest (and shortest) path algorithms. Path cost is calculated by either summing or multiplying edge weights.

## Floyd-Warshall

Floyd-Warshall algorithm has the ability to calculate longest path (ie. most profitable trade) calculations, albeit at the expensive cost of $O(n^3)$.

For multiplication based weights, we make use of the fact that product maximisation is equivalent to maximisation of log of weights, as per: $x*y = 2^{log2(x) + log2(y)}$.

For longest paths, weights have been multiplied by $-1$ and hence reused in shortest path algorithm.

_NOTE:_ Floyd-Warshall can detect negative path cycles (ie. infinite arbitrage opportunities), which cause the latest price update to be ignored. Potential TBD - remove offending edge to remove negative cycles...

Sample usage of Floyd-Warshall calculator. All prices are in $10^{12}$, including self references, eg. cost of BNB -> BNB = $10^{12}$

```rust
use best_path::prelude::*;
use best_path::prelude::floyd_warshall::calculator::FloydWarshallCalculator;

let in_graph = &[
    (
        ProviderPair { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned() },
        364_190_000_000_000_u128
    ),
    (
        ProviderPair { pair: Pair { source: "USDT".to_owned(), target: "ETH".to_owned() }, provider: "COINGECKO".to_owned() },
        2_745_000_000_u128
    ),
];
let res_out = FloydWarshallCalculator::calc_best_paths(in_graph);
let as_nodes = res_out.unwrap().into_iter().collect::<Vec<(_, _)>>();
let res_ref = res_out.as_ref().unwrap();
// multi-hop path path
assert_eq!(
    &PricePath { total_cost: 999_701_550_000_u128, steps: vec![
        PathStep { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned(), cost: 364_190_000_000_000_u128 },
        PathStep { pair: Pair { source: "USDT".to_owned(), target: "ETH".to_owned() }, provider: "COINGECKO".to_owned(), cost: 2_745_000_000_u128 }
    ] },
    res_ref.get(&Pair { source: "BNB".to_owned(), target: "ETH".to_owned() }).unwrap()
);
// 1 hop path, based on input ProviderPair
assert_eq!(
    &PricePath { total_cost: 364_190_000_000_000_u128, steps: vec![
        PathStep { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned(), cost: 364_190_000_000_000_u128 }
    ] },
    res_ref.get(&Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }).unwrap()
);
// path to self, note cost is still in scale 10^12
assert_eq!(
    &PricePath { total_cost: 1_000_000_000_000_u128, steps: vec![] },
    res_ref.get(&Pair { source: "BNB".to_owned(), target: "BNB".to_owned() }).unwrap()
);
```

### Utility within a pallet

`Best-path` serves as a best trade finding mechanism for [best-path-pallet](https://github.com/konrads/pallet-best-path).
