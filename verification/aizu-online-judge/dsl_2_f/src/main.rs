// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F

use proconio::input;
use std::cmp::min;

use lazy_segment_tree::LazySegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lst = LazySegmentTree::new(
        n,
        |a, b| min(a, b),
        (1 << 31) - 1,
        |a, b| if a == -1 { b } else { a },
        |a, b| if a == -1 { b } else { a },
        -1,
    );
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
                lst.apply(s, t + 1, x);
            }
            1 => {
                input! {
                    s: usize,
                    t: usize,
                }
                println!("{}", lst.prod(s, t + 1));
            }
            _ => unreachable!(),
        }
    }
}
