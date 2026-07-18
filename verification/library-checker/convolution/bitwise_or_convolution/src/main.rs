// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use itertools::Itertools;
use proconio::input;

use modint::Modint;
use or_convolution::or_convolution;

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
    let mut a = a.iter().map(|&a| Mint::new(a)).collect::<Vec<_>>();
    let mut b = b.iter().map(|&b| Mint::new(b)).collect::<Vec<_>>();
    a.reverse();
    b.reverse();

    let mut c = or_convolution(a, b);
    c.reverse();

    println!("{}", c.iter().join(" "));
}
