// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A

use fast_io::{Output, input, output};

use maxflow::Maxflow;

fn main() {
    input! {
        v: usize,
        e: usize,
        uvc: [(usize, usize, i64); e],
    }
    let mut out = Output::new();

    let mut mf = Maxflow::new(v);
    for (u, v, c) in uvc {
        mf.add_edge(u, v, c);
    }

    output!(out, mf.flow(0, v - 1));
}
