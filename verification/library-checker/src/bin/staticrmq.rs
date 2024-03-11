// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use std::cmp::min;

use data_structure::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        lr: [(usize, usize); q],
    }
    let mut st = SegmentTree::new(n, |a, b| min(a, b), i32::MAX);
    for i in 0..n {
        st.set(i, a[i]);
    }
    for (l, r) in lr {
        println!("{}", st.prod(l, r));
    }
}
