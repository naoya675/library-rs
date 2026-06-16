// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use proconio::input;

use lazy_segment_tree::LazySegmentTree;
use modint::Modint;

type Mint = Modint<998244353>;

query::define_query! {
    Query {
        0 => Query0(l: usize, r: usize, b: i64, c: i64),
        1 => Query1(i: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let a = a.iter().map(|&a| Mint::new(a)).collect::<Vec<_>>();
    let mut lst = LazySegmentTree::<Mint, (Mint, Mint)>::from_slice(
        &a,
        |x, _| x,
        Mint::new(0),
        |f, x| f.0 * x + f.1,
        |f, g| (f.0 * g.0, f.0 * g.1 + f.1),
        (Mint::new(1), Mint::new(0)),
    );

    for query in queries {
        match query {
            Query0(l, r, b, c) => lst.apply(l, r, (Mint::new(b), Mint::new(c))),
            Query1(i) => {
                println!("{}", lst.get(i));
            }
        }
    }
}
