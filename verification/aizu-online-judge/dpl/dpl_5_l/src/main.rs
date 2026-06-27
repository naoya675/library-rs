// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_L

use proconio::input;

use modint::Modint;
use partition_number::partition_number;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let dp = partition_number::<Mint>(n, k);

    println!("{}", if n >= k { dp[n - k] } else { Mint::new(0) });
}
