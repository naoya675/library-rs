// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use itertools::Itertools;
use proconio::{input, marker::Chars};

use z_algorithm::z_algorithm;

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", z_algorithm(&s).iter().join(" "));
}
