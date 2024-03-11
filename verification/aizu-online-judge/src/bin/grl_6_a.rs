// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A

use proconio::input;

use graph::FordFulkerson;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(usize, usize, usize); m],
    }
    let mut ff = FordFulkerson::new(n);
    for (u, v, c) in uvc {
        ff.add_edge(u, v, c);
    }
    println!("{}", ff.max_flow(0, n - 1));
}
