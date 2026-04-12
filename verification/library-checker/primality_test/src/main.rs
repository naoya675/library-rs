// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test

use proconio::input;

use miller_rabin::is_prime;

fn main() {
    input! {
        q: usize,
        n: [u64; q],
    }
    for &n in &n {
        println!("{}", if is_prime(n) { "Yes" } else { "No" });
    }
}
