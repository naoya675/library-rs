use dynamic_montgomery_modint::{DefaultId, DynamicMontgomeryModint};
use miller_rabin::is_prime;
use xorshift_64::XorShift64;

type Mint = DynamicMontgomeryModint<DefaultId>;

pub fn factorize(n: u64) -> Vec<u64> {
    let mut res = vec![];
    factorize_inner(n, &mut res);
    res.sort();
    res
}

fn factorize_inner(n: u64, res: &mut Vec<u64>) {
    if n <= 1 {
        return;
    }
    let p = pollard_rho_brent(n);
    if p == n {
        res.push(p);
        return;
    }
    factorize_inner(p, res);
    factorize_inner(n / p, res);
}

fn pollard_rho_floyd(n: u64) -> u64 {
    if n & 1 == 0 {
        return 2;
    }
    if is_prime(n) {
        return n;
    }
    Mint::set_mod(n);
    let mut rng = XorShift64::default();
    loop {
        let c = Mint::from(rng.random_range(1..n));
        let f = |x: Mint| x * x + c;
        // let mut x = Mint::from(rng.random_range(2..n)); // randomized start
        let mut x = Mint::new(2);
        let mut y = x;
        let mut g = 1;
        while g == 1 {
            x = f(x);
            y = f(f(y));
            g = gcd((x - y).value(), n);
        }
        if g != n {
            return g;
        }
    }
}

fn pollard_rho_brent(n: u64) -> u64 {
    if n & 1 == 0 {
        return 2;
    }
    if is_prime(n) {
        return n;
    }
    Mint::set_mod(n);
    let mut rng = XorShift64::default();
    let m = 1u64 << ((64 - n.leading_zeros()) / 8);
    loop {
        let c = Mint::from(rng.random_range(1..n));
        let f = |x: Mint| x * x + c;
        // let mut x = Mint::from(rng.random_range(2..n));
        let mut x = Mint::new(2);
        let mut y = x;
        let mut ys = y;
        let mut q = Mint::new(1);
        let mut r = 1;
        let mut g = 1;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..m.min(r - k) {
                    y = f(y);
                    q *= x - y;
                }
                g = gcd(q.value(), n);
                k += m;
            }
            r <<= 1;
        }
        if g == n {
            g = 1;
            while g == 1 {
                ys = f(ys);
                g = gcd((x - ys).value(), n);
            }
        }
        if g != n {
            return g;
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
