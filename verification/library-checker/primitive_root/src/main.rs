// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primitive_root

use proconio::input;

use primitive_root::primitive_root;

fn main() {
    input! {
        q: usize,
        ps: [u64; q],
    }
    for &p in &ps {
        println!("{}", primitive_root(p));
    }
}
