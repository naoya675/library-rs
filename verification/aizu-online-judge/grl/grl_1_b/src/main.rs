// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B

use fast_io::{Output, input, output};

use bellman_ford::bellman_ford;

fn main() {
    input! {
        v: usize,
        e: usize,
        r: usize,
        std: [(usize, usize, i64); e],
    }
    let mut out = Output::new();

    let (cycle, res) = bellman_ford(v, &std, r);

    if cycle {
        output!(out, "NEGATIVE CYCLE");
        return;
    }
    for res in res {
        if res < i64::MAX {
            output!(out, res);
        } else {
            output!(out, "INF");
        }
    }
}
