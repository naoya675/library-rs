use dynamic_montgomery_modint::{DefaultId, DynamicMontgomeryModint};

type Mint = DynamicMontgomeryModint<DefaultId>;

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    if n < 4759123141 {
        miller_rabin(n, &[2, 7, 61])
    } else {
        miller_rabin(n, &[2, 325, 9375, 28178, 450775, 9780504, 1795265022])
    }
}

fn miller_rabin(n: u64, ws: &[u64]) -> bool {
    Mint::set_mod(n);

    let mut s = 0;
    let mut d = n - 1;
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }
    for &a in ws {
        if a % n == 0 {
            continue;
        }
        let mut x = Mint::from(a).pow(d);
        if x.value() != 1 {
            let mut t = 0;
            while t < s {
                if x.value() == n - 1 {
                    break;
                }
                x *= x;
                t += 1;
            }
            if t == s {
                return false;
            }
        }
    }
    true
}
