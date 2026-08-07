// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

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

    if n == 0 {
        for _ in 0..q {
            output!(out, 0);
        }
        return;
    }

    let wm = WaveletMatrix::new(&a);

    for (l, r, x) in queries {
        output!(out, wm.range_freq(l, r, x, x + 1));
    }
}
