// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear

use fast_io::{Output, input, output};

use floor_sum::floor_sum;

fn main() {
    input! {
        t: usize,
        nmab: [(i64, i64, i64, i64); t],
    }
    let mut out = Output::new();

    for (n, m, a, b) in nmab {
        output!(out, floor_sum(n, m, a, b));
    }
}
