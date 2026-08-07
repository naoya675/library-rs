// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use fast_io::{Output, input, output};

use dijkstra::dijkstra_with_prev;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        abc: [(usize, usize, i64); m],
    }
    let mut out = Output::new();

    let mut graph = vec![vec![]; n];
    for (a, b, c) in abc {
        graph[a].push((b, c));
    }

    let (dist, prev) = dijkstra_with_prev(n, &graph, s);
    if dist[t] == i64::MAX {
        output!(out, -1);
        return;
    }
    let mut res = vec![t];
    let mut cur = t;
    while let Some(p) = prev[cur] {
        res.push(p);
        cur = p;
    }
    res.reverse();

    output!(out, dist[t], res.len() - 1);
    for uv in res.windows(2) {
        output!(out, uv[0], uv[1]);
    }
}
