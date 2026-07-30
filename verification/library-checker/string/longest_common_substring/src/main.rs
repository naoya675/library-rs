// verification-helper: PROBLEM https://judge.yosupo.jp/problem/longest_common_substring

use proconio::{input, marker::Chars};

use longest_common_substring::longest_common_substring;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let ((a, b), (c, d)) = longest_common_substring(&s, &t);

    println!("{} {} {} {}", a, b, c, d);
}
