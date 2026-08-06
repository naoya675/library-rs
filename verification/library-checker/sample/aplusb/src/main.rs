// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb

use fast_io::{Output, input, output};

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let mut out = Output::new();
    output!(out, a + b);
}
