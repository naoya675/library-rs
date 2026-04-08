// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_C

use proconio::input;

use binomial::Binomial;
use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut bi = Binomial::<Mint>::with_capacity(k);
    let mut res = Mint::new(0);
    for i in 0..=k {
        let term = bi.comb(k, i) * Mint::from((k - i) as u64).pow(n as u64);
        if i % 2 == 0 {
            res += term;
        } else {
            res -= term;
        }
    }
    println!("{}", res);
}
