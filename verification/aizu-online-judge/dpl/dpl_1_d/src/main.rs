// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_D

use fast_io::{Output, input, output};

use longest_increasing_subsequence::longest_increasing_subsequence;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut out = Output::new();

    output!(out, longest_increasing_subsequence(&a, true).len());
}
