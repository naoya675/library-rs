// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_parallel_unionfind

use proconio::input;

use modint::Modint;
use range_parallel_union_find::RangeParallelUnionFind;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [i64; n],
        queries: [(usize, usize, usize); q],
    }
    let mut rpuf = RangeParallelUnionFind::new(n);
    let mut x = x.iter().map(|&x| Mint::new(x)).collect::<Vec<_>>();
    let mut ans = Mint::new(0);
    let mut res = vec![];

    for &(k, a, b) in &queries {
        rpuf.merge(a, a + k, b, b + k, |c, d| {
            ans += x[c] * x[d];
            x[c] = x[c] + x[d];
        });
        res.push(ans);
    }

    for &v in &res {
        println!("{}", v);
    }
}
