// verification-helper: PROBLEM https://judge.yosupo.jp/problem/gcd_convolution

use fast_io::{Output, input};

use gcd_convolution::gcd_convolution;
use modint::Modint;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut out = Output::new();

    let a: Vec<Mint> = std::iter::once(Mint::new(0)).chain(a.iter().map(|&a| Mint::new(a))).collect();
    let b: Vec<Mint> = std::iter::once(Mint::new(0)).chain(b.iter().map(|&b| Mint::new(b))).collect();
    let c = gcd_convolution(a, b);

    out.println_iter(c[1..].iter().map(|c| c.value()), " ");
}
