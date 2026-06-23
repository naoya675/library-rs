// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_6_D

use proconio::input;

use lower_bound::LowerBound;
use upper_bound::UpperBound;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        k: [usize; q],
    }
    for &k in &k {
        println!("{} {}", a.lower_bound(&k), a.upper_bound(&k));
    }
}
