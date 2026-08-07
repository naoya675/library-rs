// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use fast_io::{Output, define_query, input, output};

use fenwick_tree::FenwickTree;

define_query! {
    Query {
        0 => Query0(p: usize, x: i64),
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

    let mut ft = FenwickTree::from_slice(&a);

    for query in queries {
        match query {
            Query0(p, x) => {
                ft.add(p, x);
            }
            Query1(l, r) => {
                output!(out, ft.sum(l, r));
            }
        }
    }
}
