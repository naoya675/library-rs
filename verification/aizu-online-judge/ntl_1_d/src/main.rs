// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_D

use proconio::input;

use linear_sieve::LinearSieve;

fn main() {
    input! {
        n: usize,
    }
    let sieve = LinearSieve::new((n as f64).sqrt() as usize + 1);
    let mut n = n;
    let mut phi = n;
    for p in sieve.primes() {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            phi = phi / p * (p - 1);
            while n % p == 0 {
                n /= p;
            }
        }
    }
    if n > 1 {
        phi = phi / n * (n - 1);
    }
    println!("{}", phi);
}
