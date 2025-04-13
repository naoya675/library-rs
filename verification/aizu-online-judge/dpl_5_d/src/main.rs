// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D

use proconio::input;

use enumeration::Enumeration;
use modint::ModInt;

type Mint = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut e = Enumeration::<Mint>::new(n + k);
    println!("{}", e.homo(k, n));
}
