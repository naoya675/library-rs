// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use proconio::{input, marker::Chars};

use itertools::Join;
use z_algorithm::z_algorithm;

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", z_algorithm(&s).iter().join(" "));
}
