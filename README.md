# best-path

[![build](../../workflows/build/badge.svg)](../../actions/workflows/build.yml)

Rust `no_std` library for finding the most profitable trade route through a currency exchange graph. Edge weights represent exchange rates; the library finds the path that maximises the product of rates along the route.

## Installation

```toml
[dependencies]
best-path = "0.1"

# Substrate / SCALE codec support (optional):
best-path = { version = "0.1", features = ["scale"] }
```

## Floyd-Warshall — all pairs

Calculates the best path between **every** pair of currencies in a single O(V³) pass.

Uses the log trick: maximising the product x·y is equivalent to maximising log₂(x) + log₂(y), so rates are transformed to –log₂(rate) and a shortest-path core is applied. Detects negative cycles (infinite arbitrage) and returns `NegativeCyclesError`.

All prices are integers scaled by 10¹², including self-references (BNB→BNB = 10¹²).

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
let res = FloydWarshallCalculator::calc_best_paths(in_graph).unwrap();

// multi-hop path: BNB → USDT → ETH
assert_eq!(
    res.get(&Pair { source: "BNB".to_owned(), target: "ETH".to_owned() }).unwrap(),
    &PricePath { total_cost: 999_701_550_000_u128, steps: vec![
        PathStep { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned(), cost: 364_190_000_000_000_u128 },
        PathStep { pair: Pair { source: "USDT".to_owned(), target: "ETH".to_owned() }, provider: "COINGECKO".to_owned(), cost: 2_745_000_000_u128 }
    ] }
);

// direct path
assert_eq!(
    res.get(&Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }).unwrap(),
    &PricePath { total_cost: 364_190_000_000_000_u128, steps: vec![
        PathStep { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned(), cost: 364_190_000_000_000_u128 }
    ] }
);

// self-reference cost is 10^12
assert_eq!(
    res.get(&Pair { source: "BNB".to_owned(), target: "BNB".to_owned() }).unwrap(),
    &PricePath { total_cost: 1_000_000_000_000_u128, steps: vec![] }
);
```

## Bellman-Ford — single source

Calculates the best paths from **one** source currency to all reachable targets at O(VE) cost. Prefer this over Floyd-Warshall when only one source is needed — it is typically 10–50× faster on sparse graphs.

```rust
use best_path::prelude::*;
use best_path::prelude::bellman_ford::calculator::BellmanFordCalculator;

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
let source = "BNB".to_owned();
let res = BellmanFordCalculator::calc_best_paths_from(&source, in_graph).unwrap();

// multi-hop path: BNB → USDT → ETH
assert_eq!(
    res.get(&Pair { source: "BNB".to_owned(), target: "ETH".to_owned() }).unwrap(),
    &PricePath { total_cost: 999_701_550_000_u128, steps: vec![
        PathStep { pair: Pair { source: "BNB".to_owned(), target: "USDT".to_owned() }, provider: "CRYPTO_COMPARE".to_owned(), cost: 364_190_000_000_000_u128 },
        PathStep { pair: Pair { source: "USDT".to_owned(), target: "ETH".to_owned() }, provider: "COINGECKO".to_owned(), cost: 2_745_000_000_u128 }
    ] }
);
```

## Utility within a pallet

`best-path` serves as a best trade finding mechanism for [best-path-pallet](https://github.com/konrads/pallet-best-path).
