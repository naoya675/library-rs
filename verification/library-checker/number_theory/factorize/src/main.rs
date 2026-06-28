// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use proconio::input;

use itertools::Join;
use pollard_rho::factorize;

fn main() {
    input! {
        q: usize,
        a: [u64; q],
    }
    for &a in &a {
        let res = factorize(a);
        println!("{} {}", res.len(), res.iter().join(" "));
    }
}
