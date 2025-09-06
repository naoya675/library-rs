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
    std.iter().for_each(|&(s, t, d)| dij.add_edge(s, t, d));

    let res = dij.dijkstra(r);
    for i in 0..v {
        println!("{}", if res[i] < i64::MAX / 4 { res[i].to_string() } else { "INF".to_string() });
    }
}
