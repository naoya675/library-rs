// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F

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
    let init = vec![(1 << 31) - 1; n];
    let mut lst = LazySegmentTree::<i64, Option<i64>>::from_slice(&init, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f.unwrap_or(x), |f, g| f.or(g), None);

    for query in queries {
        match query {
            Query0(s, t, x) => lst.apply(s, t + 1, Some(x)),
            Query1(s, t) => {
                println!("{}", lst.prod(s, t + 1));
            }
        }
    }
}
