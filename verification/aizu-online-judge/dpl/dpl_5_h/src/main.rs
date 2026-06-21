// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_H

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    println!("{}", if n <= k { 1 } else { 0 });
}
