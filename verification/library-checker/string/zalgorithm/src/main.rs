// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use fast_io::{Output, input};

use z_algorithm::z_algorithm;

fn main() {
    input! {
        s: Chars,
    }
    let mut out = Output::new();

    out.println_iter(z_algorithm(&s), " ");
}
