// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use fast_io::{Output, input, output};

use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize); q],
    }
    let mut out = Output::new();

    let st = SegmentTree::from_slice(&a, |x, y| std::cmp::min(x, y), i64::MAX);

    for (l, r) in queries {
        output!(out, st.prod(l, r));
    }
}
