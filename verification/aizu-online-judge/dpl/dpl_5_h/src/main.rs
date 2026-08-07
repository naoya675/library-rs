// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_H

use fast_io::{Output, input, output};

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut out = Output::new();

    output!(out, if n <= k { 1 } else { 0 });
}
