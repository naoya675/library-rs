// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use proconio::input;

use segment_tree::SegmentTree;

query::define_query! {
    Query {
        0 => Query0(x: usize, y: i64),
        1 => Query1(x: usize, y: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut st = SegmentTree::<i64>::new(n, |x, y| x + y, 0);

    for query in queries {
        match query {
            Query0(x, y) => st.apply(x - 1, y),
            Query1(x, y) => {
                println!("{}", st.prod(x - 1, y));
            }
        }
    }
}
