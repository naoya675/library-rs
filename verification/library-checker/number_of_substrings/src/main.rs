// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings

use proconio::{input, marker::Chars};

use lcp_array::LCPArray;
use suffix_array::SuffixArray;

fn main() {
    input! {
        s: Chars,
    }
    let sa = SuffixArray::suffix_array(&s);
    let lcp = LCPArray::lcp_array(&s, &sa);

    println!("{}", s.len() * (s.len() + 1) / 2 - lcp.iter().sum::<usize>());
}
