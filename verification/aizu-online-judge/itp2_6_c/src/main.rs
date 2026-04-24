// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_6_C

use proconio::input;

use lower_bound::LowerBound;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        k: [usize; q],
    }
    for &k in &k {
        println!("{}", a.lower_bound(&k));
    }
}
