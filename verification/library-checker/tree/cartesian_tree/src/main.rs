// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree

use proconio::input;

use cartesian_tree::CartesianTree;
use itertools::Join;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ct = CartesianTree::new(&a);

    println!("{}", ct.run(true).iter().enumerate().map(|(i, p)| p.unwrap_or(i)).join(" "));
}
