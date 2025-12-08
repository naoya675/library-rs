// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G

use proconio::input;

use lazy_segment_tree::LazySegmentTree;

query::define_query! {
    Query {
        0 => Query0(s: usize, t: usize, x: i64),
        1 => Query1(s: usize, t: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut lst = LazySegmentTree::<(i64, i64), i64>::new(n, |x, y| (x.0 + y.0, x.1 + y.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, g| f + g, 0);
    lst.build(vec![(0, 1); n]);

    for query in queries {
        match query {
            Query0(s, t, x) => lst.apply(s - 1, t, x),
            Query1(s, t) => {
                println!("{}", lst.prod(s - 1, t).0);
            }
        }
    }
}
