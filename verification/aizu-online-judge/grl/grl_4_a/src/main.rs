// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_A

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

    output!(out, if topological_sort(v, &graph).is_none() { 1 } else { 0 });
}
