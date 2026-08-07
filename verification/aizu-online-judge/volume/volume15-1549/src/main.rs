// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1549

use fast_io::{Output, input, output};

use wavelet_matrix::WaveletMatrix;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        queries: [(usize, usize, i64); q],
    }
    let mut out = Output::new();

    let offset = 1 << 20;
    let a = a.iter().map(|&a| (a + offset) as u64).collect::<Vec<_>>();
    let wm = WaveletMatrix::new(&a);

    for (l, r, d) in queries {
        let d = (d + offset) as u64;
        let prev = wm.prev_value(l, r + 1, d + 1);
        let next = wm.next_value(l, r + 1, d);
        let res = match (prev, next) {
            (Some(p), Some(n)) => (d - p).min(n - d),
            (Some(p), None) => d - p,
            (None, Some(n)) => n - d,
            (None, None) => unreachable!(),
        };
        output!(out, res);
    }
}
