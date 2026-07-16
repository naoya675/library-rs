use linear_sieve::LinearSieve;

pub fn divisor_zeta<T: Copy + std::ops::AddAssign>(a: &mut [T]) {
    let n = a.len() - 1;
    let sieve = LinearSieve::new(n);
    for p in sieve.primes() {
        for k in 1..=n / p {
            let t = a[k];
            a[k * p] += t;
        }
    }
}

pub fn divisor_mobius<T: Copy + std::ops::SubAssign>(a: &mut [T]) {
    let n = a.len() - 1;
    let sieve = LinearSieve::new(n);
    for p in sieve.primes() {
        for k in (1..=n / p).rev() {
            let t = a[k];
            a[k * p] -= t;
        }
    }
}

pub fn multiple_zeta<T: Copy + std::ops::AddAssign>(a: &mut [T]) {
    let n = a.len() - 1;
    let sieve = LinearSieve::new(n);
    for p in sieve.primes() {
        for k in (1..=n / p).rev() {
            let t = a[k * p];
            a[k] += t;
        }
    }
}

pub fn multiple_mobius<T: Copy + std::ops::SubAssign>(a: &mut [T]) {
    let n = a.len() - 1;
    let sieve = LinearSieve::new(n);
    for p in sieve.primes() {
        for k in 1..=n / p {
            let t = a[k * p];
            a[k] -= t;
        }
    }
}
