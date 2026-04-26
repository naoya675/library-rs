// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_set_range_composite

use proconio::input;

use lazy_segment_tree::LazySegmentTree;
use modint::Modint;

type Mint = Modint<998244353>;

query::define_query! {
    Query {
        0 => Query0(l: usize, r: usize, c: i64, d: i64),
        1 => Query1(l: usize, r: usize, x: i64),
    }
}

fn op_affine(x: (Mint, Mint), y: (Mint, Mint)) -> (Mint, Mint) {
    let (a1, b1) = x;
    let (a2, b2) = y;
    (a1 * a2, b1 * a2 + b2)
}

fn pow_affine(f: (Mint, Mint), mut n: u64) -> (Mint, Mint) {
    let mut value = f;
    let mut res = (Mint::new(1), Mint::new(0));
    while n > 0 {
        if n & 1 == 1 {
            res = op_affine(res, value);
        }
        value = op_affine(value, value);
        n >>= 1;
    }
    res
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(i64, i64); n],
        queries: [Query; q],
    }
    let mut lst = LazySegmentTree::<((Mint, Mint), u64), Option<(Mint, Mint)>>::new(
        n,
        |x, y| {
            let ((a1, b1), l1) = x;
            let ((a2, b2), l2) = y;
            (op_affine((a1, b1), (a2, b2)), l1 + l2)
        },
        ((Mint::new(1), Mint::new(0)), 0),
        |f, x| match f {
            Some(f) => (pow_affine(f, x.1), x.1),
            None => x,
        },
        |f, g| f.or(g),
        None,
    );
    let ab = ab.iter().map(|&(a, b)| ((Mint::new(a), Mint::new(b)), 1)).collect::<Vec<_>>();
    lst.build(&ab);

    for query in queries {
        match query {
            Query0(l, r, c, d) => {
                lst.apply(l, r, Some((Mint::new(c), Mint::new(d))));
            }
            Query1(l, r, x) => {
                let ((a, b), _) = lst.prod(l, r);
                println!("{}", Mint::new(x) * a + b);
            }
        }
    }
}
