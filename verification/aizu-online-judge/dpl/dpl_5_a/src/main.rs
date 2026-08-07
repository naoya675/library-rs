// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_A

use fast_io::{Output, input, output};

use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    let mut out = Output::new();

    output!(out, Mint::from(k).pow(n).value());
}
