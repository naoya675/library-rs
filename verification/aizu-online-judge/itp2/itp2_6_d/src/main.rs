// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_6_D

use fast_io::{Output, input, output};

use lower_bound::LowerBound;
use upper_bound::UpperBound;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        k: [i64; q],
    }
    let mut out = Output::new();

    for k in k {
        output!(out, a.lower_bound(&k), a.upper_bound(&k));
    }
}
