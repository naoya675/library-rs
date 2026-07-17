// verification-helper: PROBLEM https://judge.yosupo.jp/problem/subset_convolution

use itertools::Itertools;
use proconio::input;

use modint::Modint;
use subset_convolution::subset_convolution;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
    }
    let sz = 1 << n;
    input! {
        a: [u64; sz],
        b: [u64; sz],
    }
    let a: Vec<Mint> = a.iter().map(|&v| Mint::new(v as i64)).collect();
    let b: Vec<Mint> = b.iter().map(|&v| Mint::new(v as i64)).collect();

    let c = subset_convolution(&a, &b);

    println!("{}", c.iter().join(" "));
}
