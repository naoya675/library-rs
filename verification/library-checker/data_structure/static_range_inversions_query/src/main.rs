// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use fast_io::{Output, input, output};

use fenwick_tree::FenwickTree;
use lower_bound::LowerBound;
use mo::Mo;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let mut out = Output::new();

    let mut x = a.clone();
    x.sort();
    x.dedup();
    let a: Vec<usize> = a.iter().map(|&a| x.lower_bound(&a)).collect();
    let mut mo = Mo::new(n, q);
    for (l, r) in lr {
        mo.add_query(l, r);
    }

    struct State {
        ft: FenwickTree<i64>,
        inv: i64, // number of inversions
    }

    let mut state = State {
        ft: FenwickTree::new(x.len()),
        inv: 0,
    };
    let mut res = vec![0; q];
    mo.run_queries(
        &mut state,
        |state, i| {
            state.inv += state.ft.sum(0, a[i]);
            state.ft.add(a[i], 1);
        },
        |state, i| {
            state.inv += state.ft.sum(a[i] + 1, x.len());
            state.ft.add(a[i], 1);
        },
        |state, i| {
            state.ft.add(a[i], -1);
            state.inv -= state.ft.sum(0, a[i]);
        },
        |state, i| {
            state.ft.add(a[i], -1);
            state.inv -= state.ft.sum(a[i] + 1, x.len());
        },
        |state, idx| {
            res[idx] = state.inv;
        },
    );

    for res in res {
        output!(out, res);
    }
}
