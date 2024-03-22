// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G

use proconio::input;

use lazy_segment_tree::LazySegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lst = LazySegmentTree::<(i64, i64), i64>::new(
        n,
        |a, b| (a.0 + b.0, a.1 + b.1),
        (0, 0),
        |a, b| (b.0 + a * b.1, b.1),
        |a, b| a + b,
        0,
    );
    for i in 0..n {
        lst.set(i, (0, 1));
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
                lst.apply(s - 1, t, x);
            }
            1 => {
                input! {
                    s: usize,
                    t: usize,
                }
                println!("{}", lst.prod(s - 1, t).0);
            }
            _ => unreachable!(),
        }
    }
}
