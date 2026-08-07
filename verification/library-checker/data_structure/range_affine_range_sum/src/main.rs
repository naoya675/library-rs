// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use fast_io::{Output, define_query, input, output};

use lazy_segment_tree::LazySegmentTree;
use modint::Modint;

type Mint = Modint<998244353>;

define_query! {
    Query {
        0 => Query0(l: usize, r: usize, b: i64, c: i64),
        1 => Query1(l: usize, r: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let a = a.iter().map(|&a| (Mint::new(a), Mint::new(1))).collect::<Vec<_>>();
    let mut lst = LazySegmentTree::from_slice(
        &a,
        |x, y| (x.0 + y.0, x.1 + y.1),
        (Mint::new(0), Mint::new(0)),
        |f, x| (f.0 * x.0 + f.1 * x.1, x.1),
        |f, g| (f.0 * g.0, f.0 * g.1 + f.1),
        (Mint::new(1), Mint::new(0)),
    );

    for query in queries {
        match query {
            Query0(l, r, b, c) => lst.apply(l, r, (Mint::new(b), Mint::new(c))),
            Query1(l, r) => {
                output!(out, lst.prod(l, r).0.value());
            }
        }
    }
}
