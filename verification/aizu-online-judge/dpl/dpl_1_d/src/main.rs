// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_D

use proconio::input;

use longest_increasing_subsequence::longest_increasing_subsequence;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", longest_increasing_subsequence(&a, true).len());
}
