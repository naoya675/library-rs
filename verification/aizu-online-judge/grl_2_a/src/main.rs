// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A

use proconio::input;

use kruskal::minimum_spanning_tree;

fn main() {
    input! {
        v: usize,
        e: usize,
        mut stw: [(usize, usize, i64); e],
    }
    println!("{}", minimum_spanning_tree(v, &mut stw).0);
}
