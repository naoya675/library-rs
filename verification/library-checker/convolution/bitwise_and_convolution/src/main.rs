// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use itertools::Itertools;
use proconio::input;

use modint::Modint;
use zeta_mobius::{superset_mobius, superset_zeta};

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
    let mut a: Vec<Mint> = a.iter().map(|&v| Mint::new(v as i64)).collect();
    let mut b: Vec<Mint> = b.iter().map(|&v| Mint::new(v as i64)).collect();

    superset_zeta(&mut a);
    superset_zeta(&mut b);
    let mut c: Vec<Mint> = (0..sz).map(|i| a[i] * b[i]).collect();
    superset_mobius(&mut c);

    println!("{}", c.iter().join(" "));
}
