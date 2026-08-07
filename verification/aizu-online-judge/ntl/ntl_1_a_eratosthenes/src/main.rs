// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_A

use fast_io::{Output, input};

use eratosthenes_factorization::EratosthenesFactorization;

fn main() {
    input! {
        mut n: usize,
    }
    let mut out = Output::new();

    let sieve = EratosthenesFactorization::new((n as f64).sqrt() as usize + 1);
    let mut factors = vec![];
    for &p in sieve.primes() {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    if n > 1 {
        factors.push(n);
    }

    let n = factors.iter().product::<usize>();
    out.print(n);
    out.print(": ");
    out.println_iter(&factors, " ");
}
