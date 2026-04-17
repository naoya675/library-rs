// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings

use proconio::{input, marker::Chars};

use lcp_array::lcp_array;
use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
    }
    let lcp = lcp_array(&s, &suffix_array(&s));

    println!("{}", s.len() * (s.len() + 1) / 2 - lcp.iter().sum::<usize>());
}
