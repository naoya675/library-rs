// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_A

use proconio::input;

use montgomery_modint::MontgomeryModint;

type Mint = MontgomeryModint<1000000007>;

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    println!("{}", Mint::from(k).pow(n));
}
