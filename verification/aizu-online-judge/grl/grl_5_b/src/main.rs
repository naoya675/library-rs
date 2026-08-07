// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_B

use fast_io::{Output, input, output};

use rerooting::Rerooting;

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    input! {
        n: usize,
        stw: [(usize, usize, i64); n - 1],
    }
    let mut out = Output::new();

    let mut g = Rerooting::new(n, |x, y| std::cmp::max(x, y), || 0, |_| 0, |x, _, _, w| x + w);
    for (s, t, w) in stw {
        g.add_edge(s, t, w);
        g.add_edge(t, s, w);
    }

    for res in g.run() {
        output!(out, res);
    }
}
