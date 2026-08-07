// verification-helper: IGNORE https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_B

use fast_io::{Output, input, output};

use topological_sort::topological_sort;

fn main() {
    input! {
        v: usize,
        e: usize,
        st: [(usize, usize); e],
    }
    let mut out = Output::new();

    let mut graph = vec![vec![]; v];
    for (s, t) in st {
        graph[s].push(t);
    }

    for v in topological_sort(v, &graph).unwrap() {
        output!(out, v);
    }
}
