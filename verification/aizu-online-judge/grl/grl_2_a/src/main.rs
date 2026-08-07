// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A

use fast_io::{Output, input, output};

use kruskal::minimum_spanning_tree;

fn main() {
    input! {
        v: usize,
        e: usize,
        mut stw: [(usize, usize, i64); e],
    }
    let mut out = Output::new();

    output!(out, minimum_spanning_tree(v, &mut stw).0);
}
