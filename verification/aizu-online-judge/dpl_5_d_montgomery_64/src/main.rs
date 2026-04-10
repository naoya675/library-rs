// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D

use proconio::input;

use binomial::Binomial;
use montgomery_modint_64::MontgomeryModint64;

type Mint = MontgomeryModint64<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut bi = Binomial::<Mint>::with_capacity(n + k);
    println!("{}", bi.homo(k, n));
}
