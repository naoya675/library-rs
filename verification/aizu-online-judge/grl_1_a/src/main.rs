// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A

use proconio::input;

use dijkstra::Dijkstra;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let mut dijkstra = Dijkstra::new(v);
    for (s, t, d) in std {
        dijkstra.add_edge(s, t, d);
    }
    let res = dijkstra.dijkstra(r);
    for i in 0..v {
        if res[i] < Dijkstra::INF {
            println!("{}", res[i])
        } else {
            println!("INF");
        }
    }
}
