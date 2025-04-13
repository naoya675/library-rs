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
    let mut dij = Dijkstra::new(v);
    for (s, t, d) in std {
        dij.add_edge(s, t, d);
    }
    let res = dij.dijkstra(r);
    for i in 0..v {
        if res[i] < i64::MAX / 4 {
            println!("{}", res[i])
        } else {
            println!("INF");
        }
    }
}
