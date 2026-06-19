// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_A

use proconio::{input, marker::Chars};

use kmp::kmp;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    for i in kmp(&t, &p) {
        println!("{}", i);
    }
}
