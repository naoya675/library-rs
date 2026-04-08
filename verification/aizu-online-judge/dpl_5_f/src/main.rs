// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_F

use proconio::input;

use binomial::Binomial;
use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut bi = Binomial::<Mint>::with_capacity(n.max(k));
    println!("{}", bi.comb(n - 1, k - 1));
}
