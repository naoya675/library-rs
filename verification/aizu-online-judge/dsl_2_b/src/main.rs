// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use proconio::input;

use fenwick_tree::FenwickTree;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut ft = FenwickTree::<i64>::new(n);
    let mut st = SegmentTree::<i64>::new(n, |a, b| a + b, 0);
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { x: usize, y: i64, }
                ft.add(x - 1, y);
                st.apply(x - 1, y);
            }
            1 => {
                input! { x: usize, y: usize, }
                println!("{}", ft.sum(x - 1, y));
                assert!(ft.sum(x - 1, y) == st.prod(x - 1, y));
            }
            _ => unreachable!(),
        }
    }
}
