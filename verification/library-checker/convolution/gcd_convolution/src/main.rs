// verification-helper: PROBLEM https://judge.yosupo.jp/problem/gcd_convolution

use itertools::Itertools;
use proconio::input;

use gcd_convolution::gcd_convolution;
use modint::Modint;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }
    let a: Vec<Mint> = std::iter::once(Mint::new(0))
        .chain(a.iter().map(|&v| Mint::new(v as i64)))
        .collect();
    let b: Vec<Mint> = std::iter::once(Mint::new(0))
        .chain(b.iter().map(|&v| Mint::new(v as i64)))
        .collect();
    let c = gcd_convolution(a, b);

    println!("{}", c[1..].iter().join(" "));
}
