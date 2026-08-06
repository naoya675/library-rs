// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use fast_io::{Output, input, output};

fn main() {
    input! {
        t: usize,
    }
    let mut out = Output::new();

    for _ in 0..t {
        input! {
            a: i128,
            b: i128,
        }
        output!(out, a + b);
    }
}
