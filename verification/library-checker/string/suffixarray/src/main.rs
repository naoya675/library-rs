// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use proconio::{input, marker::Chars};

use itertools::Join;
use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
    }
    let sa = suffix_array(&s);

    println!("{}", sa.iter().join(" "));
}
