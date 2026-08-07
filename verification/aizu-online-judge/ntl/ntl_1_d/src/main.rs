// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_D

use fast_io::{Output, input, output};

use euler_phi::euler_phi;

fn main() {
    input! {
        n: usize,
    }
    let mut out = Output::new();

    output!(out, euler_phi(n));
}
