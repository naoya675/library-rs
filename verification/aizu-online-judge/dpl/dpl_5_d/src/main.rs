// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D

use fast_io::{Output, input, output};

use binomial::Binomial;
use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut out = Output::new();

    let mut bi = Binomial::<Mint>::new();

    output!(out, bi.homo(k, n).value());
}
