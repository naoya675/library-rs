// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_E

use fast_io::{Output, define_query, input, output};

use dual_segment_tree::DualSegmentTree;

define_query! {
    Query {
        0 => Query0(s: usize, t: usize, x: i64),
        1 => Query1(i: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut ds = DualSegmentTree::new(n, 0, |f, x| f + x, |f, g| f + g, 0);

    for query in queries {
        match query {
            Query0(s, t, x) => ds.apply(s - 1, t, x),
            Query1(i) => {
                output!(out, ds.get(i - 1));
            }
        }
    }
}
