// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use itertools::Itertools;
use proconio::{input, marker::Chars};

use z_algorithm::ZAlgorithm;

fn main() {
    input! {
        s: Chars,
    }
    let mut za = ZAlgorithm::<char>::new();
    za.build(&s);
    println!("{}", za.get().iter().join(" "));
}
