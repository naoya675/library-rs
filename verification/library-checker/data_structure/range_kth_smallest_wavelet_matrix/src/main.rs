// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest

use fast_io::{Output, input, output};

use wavelet_matrix::WaveletMatrix;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(usize, usize, usize); q],
    }
    let mut out = Output::new();

    let wm = WaveletMatrix::new(&a);

    for (l, r, k) in queries {
        output!(out, wm.kth_smallest(l, r, k));
    }
}
