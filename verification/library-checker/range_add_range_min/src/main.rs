// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_add_range_min

use proconio::input;

use lazy_segment_tree::LazySegmentTree;

query::define_query! {
    Query {
        0 => Query0(l: usize, r: usize, x: i64),
        1 => Query1(l: usize, r: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let mut lst = LazySegmentTree::<i64, i64>::from_slice(&a, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f + x, |f, g| f + g, 0);

    for query in queries {
        match query {
            Query0(l, r, x) => lst.apply(l, r, x),
            Query1(l, r) => {
                println!("{}", lst.prod(l, r));
            }
        }
    }
}
