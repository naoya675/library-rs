// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use proconio::input;

use lazy_segment_tree::LazySegmentTree;
use modint::StaticModint;

type Mint = StaticModint<998244353>;

query::define_query! {
    Query {
        0 => Query0(l: usize, r: usize, b: u64, c: u64),
        1 => Query1(l: usize, r: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [Query; q],
    }
    let mut lst = LazySegmentTree::<(Mint, Mint), (Mint, Mint)>::new(
        n,
        |x, y| (x.0 + y.0, x.1 + y.1),
        (Mint::new(0), Mint::new(0)),
        |f, x| (f.0 * x.0 + f.1 * x.1, x.1),
        |f, g| (f.0 * g.0, f.0 * g.1 + f.1),
        (Mint::new(1), Mint::new(0)),
    );
    let a = a.iter().map(|&a| (Mint::new(a), Mint::new(1))).collect::<Vec<_>>();
    lst.build(a);

    for query in queries {
        match query {
            Query0(l, r, b, c) => lst.apply(l, r, (Mint::new(b), Mint::new(c))),
            Query1(l, r) => {
                println!("{}", lst.prod(l, r).0);
            }
        }
    }
}
