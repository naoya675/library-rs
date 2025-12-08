// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use itertools::Itertools;
use proconio::{input, marker::Chars};

use suffix_array::SuffixArray;

fn main() {
    input! {
        s: Chars,
    }
    let sa = SuffixArray::suffix_array(&s);

    println!("{}", sa.iter().join(" "));
}
