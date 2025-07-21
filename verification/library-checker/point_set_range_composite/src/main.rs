// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use proconio::input;

use modint::StaticModint;
use segment_tree::SegmentTree;

type Mint = StaticModint<998244353>;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(u64, u64); n],
    }
    let mut st = SegmentTree::<(Mint, Mint)>::new(n, |a, b| (a.0 * b.0, a.1 * b.0 + b.1), (Mint::new(1), Mint::new(0)));
    let ab = ab.iter().map(|&(a, b)| (Mint::new(a), Mint::new(b))).collect::<Vec<_>>();
    st.build(ab);
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { p: usize, c: u64, d: u64, }
                st.set(p, (Mint::new(c), Mint::new(d)));
            }
            1 => {
                input! { l: usize, r: usize, x: u64, }
                let (a, b) = st.prod(l, r);
                println!("{}", Mint::new(x) * a + b);
            }
            _ => unreachable!(),
        }
    }
}
