// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primitive_root

use proconio::input;

use primitive_root::primitive_root;

fn main() {
    input! {
        q: usize,
        p: [u64; q],
    }
    for &p in &p {
        println!("{}", primitive_root(p));
    }
}
