// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_I

use proconio::input;

use modint::Modint;
use stirling_number_second::stirling_number_second;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    println!("{}", stirling_number_second::<Mint>(n, k));
}
