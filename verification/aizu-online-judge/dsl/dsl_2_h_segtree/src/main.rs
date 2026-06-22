// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_H

use proconio::input;

use segment_tree::SegmentTree;

#[derive(Debug, Clone, Copy)]
struct S {
    min: i64,
    sum: i64,
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
    let init = vec![S { min: 0, sum: 0 }; n + 1];
    let mut seg = SegmentTree::from_slice(
        &init,
        |x, y| S {
            min: std::cmp::min(x.min, x.sum.saturating_add(y.min)),
            sum: x.sum.saturating_add(y.sum),
        },
        S { min: i64::MAX, sum: 0 },
    );

    for query in queries {
        match query {
            Query0(s, t, x) => {
                let cur = seg.get(s);
                seg.set(
                    s,
                    S {
                        min: cur.min + x,
                        sum: cur.sum + x,
                    },
                );
                let cur = seg.get(t + 1);
                seg.set(
                    t + 1,
                    S {
                        min: cur.min - x,
                        sum: cur.sum - x,
                    },
                );
            }
            Query1(s, t) => {
                let sum = seg.prod(0, s).sum;
                let min = seg.prod(s, t + 1).min;
                println!("{}", sum + min);
            }
        }
    }
}
