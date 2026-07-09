// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_E

use proconio::input;

use ext_gcd::ext_gcd;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let (_, x, y) = ext_gcd(a, b);

    println!("{} {}", x, y);
}
