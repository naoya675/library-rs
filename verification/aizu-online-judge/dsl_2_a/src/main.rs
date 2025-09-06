// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use proconio::input;

use segment_tree::SegmentTree;

query::define_query! {
    Query {
        0 => Q0(x: usize, y: i64),
        1 => Q1(x: usize, y: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut st = SegmentTree::<i64>::new(n, |x, y| std::cmp::min(x, y), i64::MAX);
    st.build(vec![(1 << 31) - 1; n]);

    for query in queries {
        match query {
            Q0(x, y) => st.set(x, y),
            Q1(x, y) => {
                println!("{}", st.prod(x, y + 1));
            }
        }
    }
}
