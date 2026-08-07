// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use fast_io::{Output, input};

use modint::Modint;
use or_convolution::or_convolution;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        a: [i64; 1 << n],
        b: [i64; 1 << n],
    }
    let mut out = Output::new();

    let mut a = a.iter().map(|&a| Mint::new(a)).collect::<Vec<_>>();
    let mut b = b.iter().map(|&b| Mint::new(b)).collect::<Vec<_>>();
    a.reverse();
    b.reverse();

    let mut c = or_convolution(a, b);
    c.reverse();

    out.println_iter(c.iter().map(|c| c.value()), " ");
}
