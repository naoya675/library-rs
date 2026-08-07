// verification-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array

use fast_io::{Output, define_query, input, output};

use dynamic_segment_tree::DynamicSegmentTree;

define_query! {
    Query {
        0 => Query0(k: usize, v: i64),
        1 => Query1(k: usize),
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut st = DynamicSegmentTree::new(1 << 60, |a, b| a + b, 0);

    for query in queries {
        match query {
            Query0(k, v) => {
                st.set(k, v);
            }
            Query1(k) => {
                output!(out, st.get(k));
            }
        }
    }
}
