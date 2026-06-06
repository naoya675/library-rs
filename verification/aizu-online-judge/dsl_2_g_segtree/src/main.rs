// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G

use proconio::input;

use segment_tree::SegmentTree;

#[derive(Debug, Clone, Copy)]
struct S {
    sum: i64,
    wsum: i64,
    len: i64,
}

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
    let init = vec![S { sum: 0, wsum: 0, len: 1 }; n + 1];
    let mut seg = SegmentTree::from_slice(
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
                let cur = seg.get(s - 1);
                seg.set(
                    s - 1,
                    S {
                        sum: cur.sum + x,
                        wsum: cur.wsum + x,
                        len: 1,
                    },
                );
                let cur = seg.get(t);
                seg.set(
                    t,
                    S {
                        sum: cur.sum - x,
                        wsum: cur.wsum - x,
                        len: 1,
                    },
                );
            }
            Query1(s, t) => {
                let sum = seg.prod(0, s - 1).sum;
                let wsum = seg.prod(s - 1, t).wsum;
                let len = (t - (s - 1)) as i64;
                println!("{}", len * sum + wsum);
            }
        }
    }
}
