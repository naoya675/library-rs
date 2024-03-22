// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_H

use proconio::input;
use std::cmp::min;

use lazy_segment_tree::LazySegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lst = LazySegmentTree::<i64, i64>::new(
        n,
        |a, b| min(a, b),
        1 << 31,
        |a, b| a + b,
        |a, b| a + b,
        0,
    );
    for i in 0..n {
        lst.set(i, 0);
    }
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            0 => {
                input! {
                    s: usize,
                    t: usize,
                    x: i64,
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
