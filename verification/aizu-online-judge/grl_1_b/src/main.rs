// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B

use proconio::input;

use bellman_ford::bellman_ford;
use bellman_ford::Edge;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let mut edge = vec![];
    for (s, t, d) in std {
        edge.push(Edge::new(s, t, d));
    }
    let (cycle, res) = bellman_ford(v, &edge, r);
    if cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for i in 0..v {
            if res[i] < i64::MAX / 2 {
                println!("{}", res[i])
            } else {
                println!("INF");
            }
        }
    }
}
