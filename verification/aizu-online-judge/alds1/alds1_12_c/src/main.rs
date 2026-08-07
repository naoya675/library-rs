// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_C

use fast_io::{Output, input, output};

use dijkstra::dijkstra;

fn main() {
    input! {
        n: usize,
        uvc: [(usize, [(usize, i64)]); n],
    }
    let mut out = Output::new();

    let mut graph = vec![vec![]; n];
    for (u, vc) in uvc {
        graph[u] = vc;
    }

    let dist = dijkstra(n, &graph, 0);

    for (i, dist) in dist.iter().enumerate() {
        output!(out, i, dist);
    }
}
