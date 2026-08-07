// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test

use fast_io::{Output, input, output};

use miller_rabin::is_prime;

fn main() {
    input! {
        q: usize,
        n: [u64; q],
    }
    let mut out = Output::new();

    for n in n {
        output!(out, if is_prime(n) { "Yes" } else { "No" });
    }
}
