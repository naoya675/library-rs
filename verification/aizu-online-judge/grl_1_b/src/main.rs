// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B

use proconio::input;

use bellman_ford::BellmanFord;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let mut bf = BellmanFord::new(v);
    std.iter().for_each(|&(s, t, d)| bf.add_edge(s, t, d));

    let (cycle, res) = bf.bellman_ford(r);
    if cycle {
        println!("NEGATIVE CYCLE");
        return;
    }
    for i in 0..v {
        println!("{}", if res[i] < i64::MAX / 4 { res[i].to_string() } else { "INF".to_string() });
    }
}
