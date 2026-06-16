// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_E

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
    let init = vec![0; n];
    let mut lst = LazySegmentTree::<usize, usize>::from_slice(&init, |x, _| x, 0, |f, x| f + x, |f, g| f + g, 0);

    for query in queries {
        match query {
            Query0(s, t, x) => lst.apply(s - 1, t, x),
            Query1(i) => {
                println!("{}", lst.get(i - 1));
            }
        }
    }
}
