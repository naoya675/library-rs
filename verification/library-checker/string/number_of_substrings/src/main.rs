// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings

use fast_io::{Output, input, output};

use lcp_array::lcp_array;
use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
    }
    let mut out = Output::new();

    let n = s.len();
    let lcp = lcp_array(&s, &suffix_array(&s));

    output!(out, n * (n + 1) / 2 - lcp.iter().sum::<usize>());
}
