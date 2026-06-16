// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;

use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize); q],
    }
    let st = SegmentTree::from_slice(&a, |x, y| std::cmp::min(x, y), usize::MAX);
    for &(l, r) in &queries {
        println!("{}", st.prod(l, r));
    }
}
