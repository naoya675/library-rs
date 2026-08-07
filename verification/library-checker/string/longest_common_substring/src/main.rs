// verification-helper: PROBLEM https://judge.yosupo.jp/problem/longest_common_substring

use fast_io::{Output, input, output};

use longest_common_substring::longest_common_substring;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut out = Output::new();

    let ((a, b), (c, d)) = longest_common_substring(&s, &t);

    output!(out, a, b, c, d);
}
