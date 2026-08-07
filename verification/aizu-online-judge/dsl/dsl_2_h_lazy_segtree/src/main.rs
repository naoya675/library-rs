// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_H

use fast_io::{Output, define_query, input, output};

use lazy_segment_tree::LazySegmentTree;

define_query! {
    Query {
        0 => Query0(s: usize, t: usize, x: i64),
        1 => Query1(s: usize, t: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let init = vec![0; n];
    let mut lst = LazySegmentTree::from_slice(&init, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f + x, |f, g| f + g, 0);

    for query in queries {
        match query {
            Query0(s, t, x) => lst.apply(s, t + 1, x),
            Query1(s, t) => {
                output!(out, lst.prod(s, t + 1));
            }
        }
    }
}
