// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use itertools::Itertools;
use proconio::{input, marker::Chars};

use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
    }
    let sa = suffix_array(&s);

    println!("{}", sa.iter().join(" "));
}
