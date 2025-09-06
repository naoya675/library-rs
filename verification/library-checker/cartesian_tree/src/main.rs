// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree

use itertools::Itertools;
use proconio::input;

use cartesian_tree::CartesianTree;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ct = CartesianTree::new(a);

    println!("{}", ct.run(true).iter().enumerate().map(|(i, &p)| if p == n { i } else { p }).join(" "));
}
