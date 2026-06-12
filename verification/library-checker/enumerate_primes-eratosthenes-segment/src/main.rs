// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes

use proconio::input;

use eratosthenes_segment::eratosthenes_segment;
use itertools::Join;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let primes = eratosthenes_segment(0, n + 1);
    let res = primes.iter().cloned().skip(b).step_by(a).collect::<Vec<_>>();
    println!("{} {}", primes.len(), res.len());
    println!("{}", res.iter().join(" "));
}
