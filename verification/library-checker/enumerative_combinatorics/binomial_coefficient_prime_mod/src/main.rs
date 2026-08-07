// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use fast_io::{Output, input, output};

use binomial::Binomial;
use dynamic_modint::{DefaultId, DynamicModint};

type Mint = DynamicModint<DefaultId>;

fn main() {
    input! {
        t: usize,
        m: usize,
        nk: [(usize, usize); t],
    }
    let mut out = Output::new();

    Mint::set_mod(m as u64);
    let mut bi = Binomial::<Mint>::new();

    for (n, k) in nk {
        output!(out, bi.comb(n, k).value());
    }
}
