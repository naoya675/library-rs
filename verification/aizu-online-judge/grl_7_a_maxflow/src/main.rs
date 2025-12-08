// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A

use proconio::input;

use maxflow::Maxflow;

fn main() {
    input! {
        x: usize,
        y: usize,
        e: usize,
        xy: [(usize, usize); e],
    }
    let xy = xy.iter().map(|&(xi, yi)| (xi, x + yi)).collect::<Vec<_>>();

    let source = x + y;
    let sink = source + 1;
    let mut mf = Maxflow::<i64>::new(sink + 1);
    (0..x).for_each(|i| {
        mf.add_edge(source, i, 1);
    });
    xy.iter().for_each(|&(x, y)| {
        mf.add_edge(x, y, 1);
    });
    (x..x + y).for_each(|i| {
        mf.add_edge(i, sink, 1);
    });

    println!("{}", mf.flow(source, sink));
}
