// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use proconio::input;

use wavelet_matrix::WaveletMatrix;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(usize, usize, u64); q],
    }
    if n == 0 {
        for _ in 0..q {
            println!("0");
        }
        return;
    }

    let wm = WaveletMatrix::new(&a);

    for &(l, r, x) in &queries {
        println!("{}", wm.range_freq(l, r, x, x + 1));
    }
}
