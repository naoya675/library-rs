// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_add_range_min

use fast_io::{Output, define_query, input, output};

use lazy_segment_tree::LazySegmentTree;

define_query! {
    Query {
        0 => Query0(l: usize, r: usize, x: i64),
        1 => Query1(l: usize, r: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut lst = LazySegmentTree::from_slice(&a, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f + x, |f, g| f + g, 0);

    for query in queries {
        match query {
            Query0(l, r, x) => lst.apply(l, r, x),
            Query1(l, r) => {
                output!(out, lst.prod(l, r));
            }
        }
    }
}
