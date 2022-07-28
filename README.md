# Longest/shortest path algorithms

[![test](https://github.com/konrads/best-path/workflows/test/badge.svg)](https://github.com/konrads/best-path/actions/workflows/test.yml)

Collection of algorithms to facilitate longest (and shortest) path algorithms. Graph edge weight accumulation is to be done as either a sum, or a product.

## Floyd-Warshall

Floyd-Warshall algorithm has the ability to calculate longest path (ie. most profitable trade) calculations, albeit at the expensive cost of O(V^3).

For multiplication based weights, we make use of the fact that product maximisation is equivalent to maximisation of log of weights, as per: `x*y = 2^(log2(x) + log2(y))`.

For longest paths, weights have been multiplied by `-1` and hence reused in shortest path algorithm.

_NOTE:_ Floyd-Warshall can detect negative path cycles (ie. infinite arbitrage opportunities), which cause the latest price update to be ignored.
Potential TBD - remove offending edge to remove negative cycles...
