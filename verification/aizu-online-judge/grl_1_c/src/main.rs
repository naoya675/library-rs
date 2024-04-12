// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C

use itertools::Itertools;

use proconio::input;

use warshall_floyd::WarshallFloyd;

fn main() {
    input! {
        v: usize,
        e: usize,
        std: [(usize, usize, i64); e],
    }
    let mut wf = WarshallFloyd::new(v);
    for (s, t, d) in std {
        wf.add_edge(s, t, d);
    }
    let (cycle, res) = wf.warshall_floyd();
    if cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for i in 0..v {
            let res = res[i]
                .iter()
                .map(|&r| {
                    if r < WarshallFloyd::INF / 2 {
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
