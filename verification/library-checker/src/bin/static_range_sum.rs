// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use proconio::input;

use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let mut st = SegmentTree::new(n, |a, b| a + b, 0);
    for i in 0..n {
        st.set(i, a[i]);
    }
    for (l, r) in lr {
        println!("{}", st.prod(l, r));
    }
}
