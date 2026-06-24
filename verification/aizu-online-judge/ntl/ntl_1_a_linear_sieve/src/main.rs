// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_A

use proconio::input;

use linear_sieve::LinearSieve;

fn main() {
    input! {
        n: usize,
    }
    let sieve = LinearSieve::new((n as f64).sqrt() as usize + 1);
    let mut n = n;
    let mut factors = vec![];
    for p in sieve.primes() {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    let n = factors.iter().product::<usize>();
    let s = factors.iter().map(|p| p.to_string()).collect::<Vec<_>>();
    println!("{}: {}", n, s.join(" "));
}
