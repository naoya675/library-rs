// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use proconio::input;

use dynamic_segment_tree::DynamicSegmentTree;

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
    let mut st = DynamicSegmentTree::<i64>::new(n, |x, y| std::cmp::min(x, y), (1 << 31) - 1);

    for query in queries {
        match query {
            Query0(x, y) => st.set(x, y),
            Query1(x, y) => {
                println!("{}", st.prod(x, y + 1));
            }
        }
    }
}
