// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B

use proconio::input;

use bellman_ford::bellman_ford;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let (cycle, res) = bellman_ford(v, &std, r);
    if cycle {
        println!("NEGATIVE CYCLE");
        return;
    }
    for i in 0..v {
        println!("{}", if res[i] < i64::MAX / 4 { res[i].to_string() } else { "INF".to_string() });
    }
}
