// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A

use proconio::input;

use ford_fulkerson::FordFulkerson;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(usize, usize, i64); m],
    }
    let mut ff = FordFulkerson::new(n);
    for (u, v, c) in uvc {
        ff.add_edge(u, v, c);
    }
    println!("{}", ff.flow(0, n - 1));
}
