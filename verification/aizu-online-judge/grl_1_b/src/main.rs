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
    for (s, t, d) in std {
        bf.add_edge(s, t, d);
    }
    let (cycle, res) = bf.bellman_ford(r);
    if cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for i in 0..v {
            if res[i] < BellmanFord::INF {
                println!("{}", res[i])
            } else {
                println!("INF");
            }
        }
    }
}
