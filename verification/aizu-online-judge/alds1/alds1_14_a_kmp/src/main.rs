// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_A

use fast_io::{Output, input, output};

use kmp::kmp;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    let mut out = Output::new();

    for i in kmp(&t, &p) {
        output!(out, i);
    }
}
