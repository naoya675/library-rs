// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C

use itertools::Itertools;

use proconio::input;

use warshall_floyd::warshall_floyd;
use warshall_floyd::Edge;

fn main() {
    input! {
        v: usize,
        e: usize,
        std: [(usize, usize, i64); e],
    }
    let mut edge = vec![];
    for (s, t, d) in std {
        edge.push(Edge::new(s, t, d));
    }
    let (cycle, res) = warshall_floyd(v, &edge);
    if cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for i in 0..v {
            let res = res[i]
                .iter()
                .map(|&r| {
                    if r < i64::MAX / 4 {
                        r.to_string()
                    } else {
                        "INF".to_string()
                    }
                })
                .join(" ");
            println!("{}", res);
        }
    }
}
