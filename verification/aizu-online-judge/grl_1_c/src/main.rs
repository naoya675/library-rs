// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C

use itertools::Itertools;
use proconio::input;

use warshall_floyd::warshall_floyd;

fn main() {
    input! {
        v: usize,
        e: usize,
        std: [(usize, usize, i64); e],
    }
    let (cycle, res) = warshall_floyd(v, &std);
    if cycle {
        println!("NEGATIVE CYCLE");
        return;
    }
    for i in 0..v {
        let res = res[i]
            .iter()
            .map(|&res| if res < i64::MAX / 8 { res.to_string() } else { "INF".to_string() })
            .join(" ");
        println!("{}", res);
    }
}
