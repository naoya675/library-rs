// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use itertools::Itertools;
use proconio::{input, marker::Chars};

use manacher::manacher;

fn main() {
    input! {
        s: Chars,
    }
    let s = Itertools::intersperse(s.iter(), &'#').collect::<Vec<_>>();

    println!("{}", manacher(&s).iter().enumerate().map(|(i, &k)| k - 1 + ((i ^ k) & 1)).join(" "));
}
