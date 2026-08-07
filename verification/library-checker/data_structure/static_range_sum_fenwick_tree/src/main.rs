// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use fast_io::{Output, input, output};

use fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize); q],
    }
    let mut out = Output::new();

    let ft = FenwickTree::from_slice(&a);

    for (l, r) in queries {
        output!(out, ft.sum(l, r));
    }
}
