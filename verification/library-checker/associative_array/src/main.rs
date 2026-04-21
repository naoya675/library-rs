// verification-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array

use proconio::input;

use dynamic_segment_tree::DynamicSegmentTree;

query::define_query! {
    Query {
        0 => Query0(k: usize, v: i64),
        1 => Query1(k: usize),
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut st = DynamicSegmentTree::<i64>::new(1 << 60, |a, b| a + b, 0);

    for query in queries {
        match query {
            Query0(k, v) => {
                st.set(k, v);
            }
            Query1(k) => {
                println!("{}", st.get(k));
            }
        }
    }
}
