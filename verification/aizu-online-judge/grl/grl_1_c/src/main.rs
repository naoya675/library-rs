// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C

use fast_io::{Output, input, output};

use warshall_floyd::warshall_floyd;

fn main() {
    input! {
        v: usize,
        e: usize,
        std: [(usize, usize, i64); e],
    }
    let mut out = Output::new();

    let (cycle, res) = warshall_floyd(v, &std);

    if cycle {
        output!(out, "NEGATIVE CYCLE");
        return;
    }
    for res in res {
        out.println_iter(res.iter().map(|&res| if res < i64::MAX / 4 { res.to_string() } else { "INF".to_string() }), " ");
    }
}
