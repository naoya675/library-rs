// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use proconio::input;

use modint::StaticModint;
use segment_tree::SegmentTree;

type Mint = StaticModint<998244353>;

query::define_query! {
    Query {
        0 => Query0(p: usize, c: u64, d: u64),
        1 => Query1(l: usize, r: usize, x: u64),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(u64, u64); n],
        queries: [Query; q],
    }
    let mut st = SegmentTree::<(Mint, Mint)>::new(n, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), (Mint::new(1), Mint::new(0)));
    let ab = ab.iter().map(|&(a, b)| (Mint::new(a), Mint::new(b))).collect::<Vec<_>>();
    st.build(ab);

    for query in queries {
        match query {
            Query0(p, c, d) => st.set(p, (Mint::new(c), Mint::new(d))),
            Query1(l, r, x) => {
                let (a, b) = st.prod(l, r);
                println!("{}", Mint::new(x) * a + b);
            }
        }
    }
}
