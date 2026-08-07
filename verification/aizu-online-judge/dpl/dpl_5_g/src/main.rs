// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_G

use fast_io::{Output, input, output};

use bell_number::bell_number;
use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut out = Output::new();

    output!(out, bell_number::<Mint>(n, k).value());
}
