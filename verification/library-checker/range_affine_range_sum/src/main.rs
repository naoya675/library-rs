// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use proconio::input;

use lazy_segment_tree::LazySegmentTree;
use mod_int::ModInt;

type Mint = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }
    let mut lst = LazySegmentTree::<(Mint, Mint), (Mint, Mint)>::new(
        n,
        |a, b| (a.0 + b.0, a.1 + b.1),
        (Mint::new(0), Mint::new(0)),
        |a, b| (a.0 * b.0 + a.1 * b.1, b.1),
        |a, b| (a.0 * b.0, a.0 * b.1 + a.1),
        (Mint::new(1), Mint::new(0)),
    );
    let a = a
        .iter()
        .map(|&a| (Mint::new(a), Mint::new(1)))
        .collect::<Vec<_>>();
    lst.build(a);
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            0 => {
                input! {
                    l: usize,
                    r: usize,
                    b: u64,
                    c: u64,
                }
                lst.apply(l, r, (Mint::new(b), Mint::new(c)));
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", lst.prod(l, r).0);
            }
            _ => unreachable!(),
        }
    }
}
