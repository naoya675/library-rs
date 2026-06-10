// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use proconio::input;

use itertools::Itertools;

use pollard_rho::factorize;

fn main() {
    input! {
        q: usize,
        a: [u64; q],
    }
    for &a in &a {
        let f = factorize(a);
        let out = std::iter::once(f.len().to_string())
            .chain(f.iter().map(|x| x.to_string()))
            .join(" ");
        println!("{}", out);
    }
}
