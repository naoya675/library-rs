// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A

use proconio::input;

use kruskal::Kruskal;

fn main() {
    input! {
        v: usize,
        e: usize,
        stw: [(usize, usize, i64); e],
    }
    let mut krs = Kruskal::new(v);
    for (s, t, w) in stw {
        krs.add_edge(s, t, w);
    }
    println!("{}", krs.minimum_spanning_tree().0);
}
