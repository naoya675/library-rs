// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_B

use fast_io::{Output, input, output};

use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        m: i64,
        n: i64,
    }
    let mut out = Output::new();

    output!(out, Mint::new(m).pow(n as u64).value());
}
