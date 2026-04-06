// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use proconio::input;

use combinatorics::Combinatorics;
use dynamic_modint::{DefaultId, DynamicModint};

type Mint = DynamicModint<DefaultId>;

fn main() {
    input! {
        t: usize,
        m: usize,
        nk: [(usize, usize); t],
    }
    Mint::set_mod(m as u64);
    let mut comb = Combinatorics::<Mint>::new((m - 1).min(nk.iter().map(|&(n, _)| n).max().unwrap()));

    for &(n, k) in &nk {
        println!("{}", comb.comb(n, k));
    }
}
