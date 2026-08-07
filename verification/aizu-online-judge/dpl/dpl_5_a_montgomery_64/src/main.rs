// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_A

use fast_io::{Output, input, output};

use montgomery_modint_64::MontgomeryModint64;

type Mint = MontgomeryModint64<1000000007>;

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    let mut out = Output::new();

    output!(out, Mint::from(k).pow(n).value());
}
