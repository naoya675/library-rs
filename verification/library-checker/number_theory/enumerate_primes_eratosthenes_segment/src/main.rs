// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes

use fast_io::{Output, input, output};

use eratosthenes_segment::eratosthenes_segment;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut out = Output::new();

    let primes = eratosthenes_segment(0, n + 1);
    let res = primes.iter().cloned().skip(b).step_by(a).collect::<Vec<_>>();

    output!(out, primes.len(), res.len());
    out.println_iter(&res, " ");
}
