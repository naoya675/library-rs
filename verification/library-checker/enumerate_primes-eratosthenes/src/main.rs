// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes

use proconio::input;

use eratosthenes::eratosthenes;
use itertools::Join;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let primes = eratosthenes(n);
    let res = primes.iter().cloned().skip(b).step_by(a).collect::<Vec<_>>();
    println!("{} {}", primes.len(), res.len());
    println!("{}", res.iter().join(" "));
}
