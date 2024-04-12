// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A

use proconio::input;

use kruskal::Kruskal;

fn main() {
    input! {
        v: usize,
        e: usize,
        stw: [(usize, usize, i64); e],
    }
    let mut kruskal = Kruskal::new(v);
    for (s, t, w) in stw {
        kruskal.add_edge(s, t, w);
    }
    println!("{}", kruskal.minimum_spanning_tree().0);
}
