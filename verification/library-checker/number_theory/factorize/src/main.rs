// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use proconio::input;

use itertools::Join;
use pollard_rho::factors_dup;

fn main() {
    input! {
        q: usize,
        a: [u64; q],
    }
    for &a in &a {
        let res = factors_dup(a);
        println!("{} {}", res.len(), res.iter().join(" "));
    }
}
