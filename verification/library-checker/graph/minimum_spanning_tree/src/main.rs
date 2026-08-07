// verification-helper: PROBLEM https://judge.yosupo.jp/problem/minimum_spanning_tree

use std::collections::HashMap;

use fast_io::{Output, input, output};

use kruskal::minimum_spanning_tree;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, i64); m],
    }
    let mut out = Output::new();

    let mut index = HashMap::new();
    for (i, &e) in abc.iter().enumerate() {
        *index.entry(e).or_default() = i;
    }

    let (x, mst) = minimum_spanning_tree(n, &mut abc);
    let used = mst.iter().map(|e| index.get(e).unwrap()).collect::<Vec<_>>();

    output!(out, x);
    out.println_iter(&used, " ");
}
