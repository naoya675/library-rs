// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A

use proconio::input;

use maxflow::Maxflow;

fn main() {
    input! {
        v: usize,
        e: usize,
        uvc: [(usize, usize, i64); e],
    }
    let mut mf = Maxflow::<i64>::new(v);
    uvc.iter().for_each(|&(u, v, c)| {
        mf.add_edge(u, v, c);
    });

    println!("{}", mf.flow(0, v - 1));
}
