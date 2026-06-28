// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use proconio::input;

use fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize); q],
    }
    let ft = FenwickTree::<i64>::from_slice(&a);

    for &(l, r) in &queries {
        println!("{}", ft.sum(l, r));
    }
}
