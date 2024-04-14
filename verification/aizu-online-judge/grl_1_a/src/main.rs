// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A

use proconio::input;

use dijkstra::dijkstra;
use dijkstra::Edge;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let mut graph = vec![vec![]; v];
    for (s, t, d) in std {
        graph[s].push(Edge::new(t, d));
    }
    let res = dijkstra(v, &graph, r);
    for i in 0..v {
        if res[i] < i64::MAX / 2 {
            println!("{}", res[i])
        } else {
            println!("INF");
        }
    }
}
