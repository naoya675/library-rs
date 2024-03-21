// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use proconio::input;

use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut st = SegmentTree::new(n, |a, b| a + b, 0);
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            0 => {
                input! {
                    x: usize,
                    y: usize,
                }
                st.apply(x - 1, y);
            }
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                println!("{}", st.prod(x - 1, y));
            }
            _ => unreachable!(),
        }
    }
}
