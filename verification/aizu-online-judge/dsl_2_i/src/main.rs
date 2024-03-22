// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I

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
        |a, b| if a == 1 << 31 { b } else { (a * b.1, b.1) },
        |a, b| if a == 1 << 31 { b } else { a },
        1 << 31,
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
                lst.apply(s, t + 1, x);
            }
            1 => {
                input! {
                    s: usize,
                    t: usize,
                }
                println!("{}", lst.prod(s, t + 1).0);
            }
            _ => unreachable!(),
        }
    }
}
