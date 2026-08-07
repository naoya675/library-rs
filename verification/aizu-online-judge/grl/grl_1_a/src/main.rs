// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A

use fast_io::{Output, input, output};

use dijkstra::dijkstra;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let mut out = Output::new();

    let mut graph = vec![vec![]; v];
    for (s, t, d) in std {
        graph[s].push((t, d));
    }

    let res = dijkstra(v, &graph, r);

    for &res in &res {
        if res < i64::MAX {
            output!(out, res);
        } else {
            output!(out, "INF");
        }
    }
}
