// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_B

use proconio::input;

use modint::Modint;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        m: i64,
        n: i64,
    }
    println!("{}", Mint::new(m).pow(n as u64));
}
