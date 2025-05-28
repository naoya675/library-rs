// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential

use proconio::input;

use modint::StaticModint;
use union_find_with_potential::UnionFindWithPotential;

type Mint = StaticModint<998244353>;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFindWithPotential::<Mint>::new_default(n);
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { u: usize, v: usize, x: u64 }
                println!("{}", if let Some(_) = uf.merge(u, v, Mint::new(x)) { 1 } else { 0 });
            }
            1 => {
                input! { u: usize, v: usize, }
                if uf.same(u, v) {
                    println!("{}", uf.diff(u, v));
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
