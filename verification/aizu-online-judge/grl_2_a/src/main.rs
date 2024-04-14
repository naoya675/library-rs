// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A

use proconio::input;

use kruskal::minimum_spanning_tree;
use kruskal::Edge;

fn main() {
    input! {
        v: usize,
        e: usize,
        stw: [(usize, usize, i64); e],
    }
    let mut edge = vec![];
    for (s, t, w) in stw {
        edge.push(Edge::new(s, t, w));
    }
    println!("{}", minimum_spanning_tree(v, &mut edge).0);
}
