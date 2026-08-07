// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_A

use fast_io::{Output, input, output};

use kruskal::minimum_spanning_tree;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }
    let mut out = Output::new();

    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            if a[i][j] != -1 {
                edges.push((i, j, a[i][j]));
            }
        }
    }

    output!(out, minimum_spanning_tree(n, &mut edges).0);
}
