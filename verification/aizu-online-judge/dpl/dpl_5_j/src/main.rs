// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_J

use proconio::input;

use modint::Modint;
use partition_number::partition_number;

type Mint = Modint<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    println!("{}", partition_number::<Mint>(n, k)[n]);
}
