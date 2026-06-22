// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A

use proconio::input;

use ford_fulkerson::FordFulkerson;

fn main() {
    input! {
        x: usize,
        y: usize,
        e: usize,
        xy: [(usize, usize); e],
    }
    let xy = xy.iter().map(|&(xi, yi)| (xi, x + yi)).collect::<Vec<_>>();
    let s = x + y;
    let t = x + y + 1;
    let mut ff = FordFulkerson::new(t + 1);
    (0..x).for_each(|i| {
        ff.add_edge(s, i, 1);
    });
    (x..x + y).for_each(|i| {
        ff.add_edge(i, t, 1);
    });
    xy.iter().for_each(|&(x, y)| {
        ff.add_edge(x, y, 1);
    });
    println!("{}", ff.flow(s, t)); // source -> sink
}
