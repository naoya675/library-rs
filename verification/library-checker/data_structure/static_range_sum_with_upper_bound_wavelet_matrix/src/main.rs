// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum_with_upper_bound

use fast_io::{Output, input, output};

use wavelet_matrix::WaveletMatrix;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(usize, usize, u64); q],
    }
    let mut out = Output::new();

    let wm = WaveletMatrix::new(&a);

    for (l, r, x) in queries {
        output!(out, wm.range_freq_less(l, r, x + 1), wm.range_sum_less(l, r, x + 1));
    }
}
