// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb

use fast_io::{Output, input, output};

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let mut out = Output::new();
    output!(out, a + b);
}
