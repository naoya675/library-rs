// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes

use fast_io::{Output, input, output};

use eratosthenes::eratosthenes;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut out = Output::new();

    let primes = eratosthenes(n);
    let res = primes.iter().cloned().skip(b).step_by(a).collect::<Vec<_>>();

    output!(out, primes.len(), res.len());
    out.println_iter(&res, " ");
}
