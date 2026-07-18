// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use itertools::Itertools;
use proconio::input;

use and_convolution::and_convolution;
use modint::Modint;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
    }
    let n = 1 << n;
    input! {
        a: [i64; n],
        b: [i64; n],
    }
    let a = a.iter().map(|&a| Mint::new(a)).collect::<Vec<_>>();
    let b = b.iter().map(|&b| Mint::new(b)).collect::<Vec<_>>();

    let c = and_convolution(a, b);

    println!("{}", c.iter().join(" "));
}
