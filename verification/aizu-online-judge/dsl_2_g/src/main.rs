// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G

use proconio::input;

use lazy_segment_tree::LazySegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lst = LazySegmentTree::<(i64, i64), i64>::new(n, |a, b| (a.0 + b.0, a.1 + b.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, g| f + g, 0);
    lst.build(vec![(0, 1); n]);
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { s: usize, t: usize, x: i64, }
                lst.apply(s - 1, t, x);
            }
            1 => {
                input! { s: usize, t: usize, }
                println!("{}", lst.prod(s - 1, t).0);
            }
            _ => unreachable!(),
        }
    }
}
