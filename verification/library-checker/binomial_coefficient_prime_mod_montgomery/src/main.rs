// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use proconio::input;

use binomial::Binomial;
use dynamic_modint::DynamicModint;
use dynamic_montgomery_modint::DynamicMontgomeryModint;

fn main() {
    input! {
        t: usize,
        m: usize,
        nk: [(usize, usize); t],
    }
    if m == 2 {
        type Mint = DynamicModint<dynamic_modint::DefaultId>;
        Mint::set_mod(m as u64);
        let mut bi = Binomial::<Mint>::new();
        for &(n, k) in &nk {
            println!("{}", bi.comb(n, k));
        }
    } else {
        type Mint = DynamicMontgomeryModint<dynamic_montgomery_modint::DefaultId>;
        Mint::set_mod(m as u64);
        let mut bi = Binomial::<Mint>::new();
        for &(n, k) in &nk {
            println!("{}", bi.comb(n, k));
        }
    }
}
