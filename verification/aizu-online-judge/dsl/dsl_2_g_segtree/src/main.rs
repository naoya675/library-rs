// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G

use fast_io::{Output, define_query, input, output};

use segment_tree::SegmentTree;

define_query! {
    Query {
        0 => Query0(s: usize, t: usize, x: i64),
        1 => Query1(s: usize, t: usize),
    }
}

#[derive(Debug, Clone, Copy)]
struct S {
    sum: i64,
    wsum: i64,
    len: i64,
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let init = vec![S { sum: 0, wsum: 0, len: 1 }; n + 1];
    let mut st = SegmentTree::from_slice(
        &init,
        |x, y| S {
            sum: x.sum + y.sum,
            wsum: x.wsum + y.len * x.sum + y.wsum,
            len: x.len + y.len,
        },
        S { sum: 0, wsum: 0, len: 0 },
    );

    for query in queries {
        match query {
            Query0(s, t, x) => {
                let cur = st.get(s - 1);
                st.set(
                    s - 1,
                    S {
                        sum: cur.sum + x,
                        wsum: cur.wsum + x,
                        len: 1,
                    },
                );
                let cur = st.get(t);
                st.set(
                    t,
                    S {
                        sum: cur.sum - x,
                        wsum: cur.wsum - x,
                        len: 1,
                    },
                );
            }
            Query1(s, t) => {
                let sum = st.prod(0, s - 1).sum;
                let wsum = st.prod(s - 1, t).wsum;
                let len = (t - (s - 1)) as i64;
                output!(out, len * sum + wsum);
            }
        }
    }
}
