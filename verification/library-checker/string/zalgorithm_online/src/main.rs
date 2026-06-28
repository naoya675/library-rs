// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use proconio::{input, marker::Chars};

use itertools::Join;
use z_algorithm_online::ZAlgorithm;

fn main() {
    input! {
        s: Chars,
    }
    let mut za = ZAlgorithm::new();
    za.build(&s);

    println!("{}", za.get().iter().join(" "));
}
