// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use proconio::{input, marker::Chars};

use itertools::Join;
use manacher::manacher;

fn main() {
    input! {
        s: Chars,
    }
    let s = s.into_iter().flat_map(|c| ['#', c]).skip(1).collect::<Vec<_>>();

    println!("{}", manacher(&s).iter().enumerate().map(|(i, &k)| k - 1 + ((i ^ k) & 1)).join(" "));
}
