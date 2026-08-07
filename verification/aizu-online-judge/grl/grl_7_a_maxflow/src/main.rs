// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A

use fast_io::{Output, input, output};

use maxflow::Maxflow;

fn main() {
    input! {
        x: usize,
        y: usize,
        e: usize,
        xy: [(usize, usize); e],
    }
    let mut out = Output::new();

    let xy = xy.iter().map(|&(xi, yi)| (xi, yi + x)).collect::<Vec<_>>();
    let s = x + y;
    let t = x + y + 1;

    let mut mf = Maxflow::new(t + 1);
    for i in 0..x {
        mf.add_edge(s, i, 1);
    }
    for i in x..x + y {
        mf.add_edge(i, t, 1);
    }
    for (x, y) in xy {
        mf.add_edge(x, y, 1);
    }

    output!(out, mf.flow(s, t)); // source -> sink
}
