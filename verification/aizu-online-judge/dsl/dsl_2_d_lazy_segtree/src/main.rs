// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_D

use proconio::input;

use lazy_segment_tree::LazySegmentTree;

query::define_query! {
    Query {
        0 => Query0(s: usize, t: usize, x: usize),
        1 => Query1(i: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut lst = LazySegmentTree::<usize, Option<usize>>::new(n, |x, _| x, (1 << 31) - 1, |f, x| f.unwrap_or(x), |f, g| f.or(g), None);

    for query in queries {
        match query {
            Query0(s, t, x) => lst.apply(s, t + 1, Some(x)),
            Query1(i) => {
                println!("{}", lst.get(i));
            }
        }
    }
}
