// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_F

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

    output!(out, bi.comb(n - 1, k - 1).value());
}
