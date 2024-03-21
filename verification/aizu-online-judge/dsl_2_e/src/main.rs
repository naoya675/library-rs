// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_E

use proconio::input;
use std::cmp::min;

use lazy_segment_tree::LazySegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lst = LazySegmentTree::new(n, |a, b| min(a, b), 0, |a, b| a + b, |a, b| a + b, 0);
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            0 => {
                input! {
                    s: usize,
                    t: usize,
                    x: isize,
                }
                lst.apply(s - 1, t, x);
            }
            1 => {
                input! {
                    i: usize,
                }
                println!("{}", lst.get(i - 1));
            }
            _ => unreachable!(),
        }
    }
}
