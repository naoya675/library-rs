// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree

use fast_io::{Output, input};

use cartesian_tree::CartesianTree;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut out = Output::new();

    let mut ct = CartesianTree::new(&a);

    out.println_iter(ct.run(true).iter().enumerate().map(|(i, p)| p.unwrap_or(i)), " ");
}
