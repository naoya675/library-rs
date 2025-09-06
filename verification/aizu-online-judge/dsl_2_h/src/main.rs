// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_H

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
    let mut lst = LazySegmentTree::<i64, i64>::new(n, |a, b| std::cmp::min(a, b), i64::MAX, |f, x| f + x, |f, g| f + g, 0);
    lst.build(vec![0; n]);

    for query in queries {
        match query {
            Query0(s, t, x) => lst.apply(s, t + 1, x),
            Query1(s, t) => {
                println!("{}", lst.prod(s, t + 1));
            }
        }
    }
}
