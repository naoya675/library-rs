// verification-helper: PROBLEM https://judge.yosupo.jp/problem/subset_convolution

use fast_io::{Output, input};

use modint::Modint;
use subset_convolution::subset_convolution;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        a: [i64; 1 << n],
        b: [i64; 1 << n],
    }
    let mut out = Output::new();

    let a: Vec<Mint> = a.iter().map(|&v| Mint::new(v)).collect();
    let b: Vec<Mint> = b.iter().map(|&v| Mint::new(v)).collect();

    let c = subset_convolution(&a, &b);

    out.println_iter(c.iter().map(|c| c.value()), " ");
}
