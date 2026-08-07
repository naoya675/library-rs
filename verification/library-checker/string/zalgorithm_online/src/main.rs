// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use fast_io::{Output, input};

use z_algorithm_online::ZAlgorithm;

fn main() {
    input! {
        s: Chars,
    }
    let mut out = Output::new();

    let mut za = ZAlgorithm::new();
    za.build(&s);

    out.println_iter(za.get(), " ");
}
