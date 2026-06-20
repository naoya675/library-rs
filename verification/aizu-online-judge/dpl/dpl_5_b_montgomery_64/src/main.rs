// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_B

use proconio::input;

use binomial::Binomial;
use montgomery_modint_64::MontgomeryModint64;

type Mint = MontgomeryModint64<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut bi = Binomial::<Mint>::new();
    println!("{}", bi.perm(k, n));
}
