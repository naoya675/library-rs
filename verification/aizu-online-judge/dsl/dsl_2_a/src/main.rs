// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use fast_io::{Output, define_query, input, output};

use segment_tree::SegmentTree;

define_query! {
    Query {
        0 => Query0(x: usize, y: i64),
        1 => Query1(x: usize, y: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut st = SegmentTree::new(n, |x, y| std::cmp::min(x, y), (1 << 31) - 1);

    for query in queries {
        match query {
            Query0(x, y) => st.set(x, y),
            Query1(x, y) => {
                output!(out, st.prod(x, y + 1));
            }
        }
    }
}
