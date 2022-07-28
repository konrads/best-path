
/// Tests make use of example and algorithm presented in: https://www.youtube.com/watch?v=oNI0rf2P9gE&ab_channel=AbdulBari
///
///         8            3
///    0. *----------------* 1.
///    * *                   |
///  2 |  5 \                |
///    |       \             |
///    |          \          |
///    |             \       |
///  7 |                \    | 2
///    *                   \ *
///    3. *----------------- 2.
///         1
///
/// Expected result (as per youtube):
///      0.  1.  2.  3.
///  0.  0   3   5   6
///  1.  5   0   2   3
///  2.  3   6   0   1
///  3.  2   5   7   0
use super::algo::*;
use core::cmp::Ordering;
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};


#[test]
fn test_unique_edges() {
    let set = unique_cheapest_edges(&vec![
        Edge{pair: Pair{source: 1, target: 2}, provider:   3, cost: 1.0},
        Edge{pair: Pair{source: 1, target: 2}, provider:  33, cost: 10.0},
        Edge{pair: Pair{source: 1, target: 2}, provider: 333, cost: 5.0},
    ], Ordering::Less);

    assert_eq!(1, set.len());
    assert_eq!(10.0, set.iter().next().unwrap().cost);
}

/// Graph
///      .5     2        .25    4
/// 0. *----------* 1. *----------* 2.
#[test]
fn test_simple() {
    let edges = vec![
        Edge{pair: Pair{source: 0, target: 1}, provider: 1, cost: 2.0},
        Edge{pair: Pair{source: 1, target: 0}, provider: 1, cost: 0.5},
        Edge{pair: Pair{source: 1, target: 2}, provider: 1, cost: 4.0},
        Edge{pair: Pair{source: 2, target: 1}, provider: 1, cost: 0.25},
    ];
    let res = longest_paths_mult(&edges).unwrap().into_iter().collect::<Vec<_>>();
    assert_eq!(
        res,
        vec![
            (Pair { source: 0, target: 0 }, Path { total_cost: 1.0,   edges: vec![] }),
            (Pair { source: 0, target: 1 }, Path { total_cost: 2.0,   edges: vec![Edge { pair: Pair{source: 0, target: 1}, provider: 1, cost: 2.0 }] }),
            (Pair { source: 0, target: 2 }, Path { total_cost: 8.0,   edges: vec![Edge { pair: Pair { source: 0, target: 1 }, provider: 1, cost: 2.0 }, Edge { pair: Pair { source: 1, target: 2 }, provider: 1, cost: 4.0 }] }),
            (Pair { source: 1, target: 0 }, Path { total_cost: 0.5,   edges: vec![Edge { pair: Pair { source: 1, target: 0 }, provider: 1, cost: 0.5 }] }),
            (Pair { source: 1, target: 1 }, Path { total_cost: 1.0,   edges: vec![] }),
            (Pair { source: 1, target: 2 }, Path { total_cost: 4.0,   edges: vec![Edge { pair: Pair { source: 1, target: 2 }, provider: 1, cost: 4.0 }] }),
            (Pair { source: 2, target: 0 }, Path { total_cost: 0.125, edges: vec![Edge { pair: Pair { source: 2, target: 1 }, provider: 1, cost: 0.25 }, Edge { pair: Pair { source: 1, target: 0 }, provider: 1, cost: 0.5 }] }),
            (Pair { source: 2, target: 1 }, Path { total_cost: 0.25,  edges: vec![Edge { pair: Pair { source: 2, target: 1 }, provider: 1, cost: 0.25 }] }),
            (Pair { source: 2, target: 2 }, Path { total_cost: 1.0,   edges: vec![] })
        ],
    );
}

#[test]
fn test_youtube() {
    let edges = vec![
        Edge{pair: Pair{source: 0, target: 1}, provider: 1, cost: 3.0},
        Edge{pair: Pair{source: 0, target: 3}, provider: 1, cost: 7.0},
        Edge{pair: Pair{source: 0, target: 3}, provider: 10, cost: 6.5},  // ignore!!!

        Edge{pair: Pair{source: 1, target: 0}, provider: 1, cost: 8.0},
        Edge{pair: Pair{source: 1, target: 2}, provider: 1, cost: 2.0},
        
        Edge{pair: Pair{source: 2, target: 0}, provider: 1, cost: 5.0},
        Edge{pair: Pair{source: 2, target: 3}, provider: 1, cost: 1.0},

        Edge{pair: Pair{source: 3, target: 0}, provider: 1, cost: 2.0},
    ];
    let res = shortest_paths(&edges).unwrap();
    let costs = (0_usize..=3).map(|source|
        (0_usize..=3).map(|target|
            res.get(&Pair{source, target}).map(|p|p.total_cost)
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    assert_eq!(vec![
        vec![Some(0.0), Some(3.0), Some(5.0), Some(6.0)],
        vec![Some(5.0), Some(0.0), Some(2.0), Some(3.0)],
        vec![Some(3.0), Some(6.0), Some(0.0), Some(1.0)],
        vec![Some(2.0), Some(5.0), Some(7.0), Some(0.0)],
        ], costs);
    assert_eq!(vec![
        Edge { pair: Pair { source: 0, target: 1 }, provider: 1, cost: 3.0 },
        Edge { pair: Pair { source: 1, target: 2 }, provider: 1, cost: 2.0 },
        Edge { pair: Pair { source: 2, target: 3 }, provider: 1, cost: 1.0 }], res[&Pair{source: 0, target: 3}].edges);
    assert_eq!(vec![
        Edge { pair: Pair { source: 3, target: 0 }, provider: 1, cost: 2.0 },], res[&Pair{source: 3, target: 0}].edges);
    }

#[test]
fn test_youtube_negative_cycle() {
    let edges = vec![
        Edge{pair: Pair{source: 0, target: 1}, provider: 1, cost: 3.0},
        Edge{pair: Pair{source: 0, target: 3}, provider: 1, cost: 7.0},
        Edge{pair: Pair{source: 0, target: 3}, provider: 10, cost: 6.0},  // ignore!!!

        Edge{pair: Pair{source: 1, target: 0}, provider: 1, cost: 8.0},
        Edge{pair: Pair{source: 1, target: 2}, provider: 1, cost: 2.0},
        
        Edge{pair: Pair{source: 2, target: 0}, provider: 1, cost: -6.0},  // causes negative cycle
        Edge{pair: Pair{source: 2, target: 3}, provider: 1, cost: 1.0},

        Edge{pair: Pair{source: 3, target: 0}, provider: 1, cost: 2.0},
    ];
    assert!(shortest_paths(&edges).is_err());
}

#[test]
/// Few tests to ensure comparability of floats
fn test_f64() {
    assert!(f64::MAX  > 1.0);
    assert!(-f64::MAX < 1.0);
    assert!(! (f64::MAX > f64::MAX + 1.0));  // neither > or < than MAX
    assert!(! (f64::MAX < f64::MAX + 1.0));
    assert_eq!(f64::MAX, f64::MAX + 1.0);
    assert_eq!(f64::MAX, f64::MAX - 1.0);
    assert!(!(f64::MAX == 8.0 + 7.0));
    assert!(f64::MAX != 8.0 + 7.0);
    assert!(f64::MAX > 8.0 + 7.0);
    assert!(! (f64::MAX <= 8.0 + 7.0));

    let max_u128 = u128::MAX;
    let max_u128_as_f64 = (max_u128 as f64) * 10.0 + 10.0;
    let max_u128_as_f64_as_u128 = max_u128_as_f64 as u128;
    assert_eq!(max_u128, max_u128_as_f64_as_u128);
}