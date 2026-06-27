// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_C

use proconio::input;

use binomial::Binomial;
use modint::Modint;
use stirling_number_second::stirling_number_second;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut bi = Binomial::<Mint>::new();

    println!("{}", bi.fact(k) * stirling_number_second::<Mint>(n, k));
}
