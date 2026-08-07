// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_B

use fast_io::{Output, input, output};

use binomial::Binomial;
use montgomery_modint::MontgomeryModint;

type Mint = MontgomeryModint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut out = Output::new();

    let mut bi = Binomial::<Mint>::new();

    output!(out, bi.perm(k, n).value());
}
