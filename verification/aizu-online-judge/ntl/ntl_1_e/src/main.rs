// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_E

use fast_io::{Output, input, output};

use ext_gcd::ext_gcd;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let mut out = Output::new();

    let (_, x, y) = ext_gcd(a, b);

    output!(out, x, y);
}
