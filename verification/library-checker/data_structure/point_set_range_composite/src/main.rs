// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use fast_io::{Output, define_query, input, output};

use modint::Modint;
use segment_tree::SegmentTree;

type Mint = Modint<998244353>;

define_query! {
    Query {
        0 => Query0(p: usize, c: i64, d: i64),
        1 => Query1(l: usize, r: usize, x: i64),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(i64, i64); n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let ab = ab.iter().map(|&(a, b)| (Mint::new(a), Mint::new(b))).collect::<Vec<_>>();
    let mut st = SegmentTree::from_slice(&ab, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), (Mint::new(1), Mint::new(0)));

    for query in queries {
        match query {
            Query0(p, c, d) => st.set(p, (Mint::new(c), Mint::new(d))),
            Query1(l, r, x) => {
                let (a, b) = st.prod(l, r);
                output!(out, (Mint::new(x) * a + b).value());
            }
        }
    }
}
