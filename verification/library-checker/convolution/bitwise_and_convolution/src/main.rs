// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use fast_io::{Output, input};

use and_convolution::and_convolution;
use modint::Modint;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        a: [i64; 1 << n],
        b: [i64; 1 << n],
    }
    let mut out = Output::new();

    let a = a.iter().map(|&a| Mint::new(a)).collect::<Vec<_>>();
    let b = b.iter().map(|&b| Mint::new(b)).collect::<Vec<_>>();

    let c = and_convolution(a, b);

    out.println_iter(c.iter().map(|c| c.value()), " ");
}
