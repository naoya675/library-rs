// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential

use proconio::input;

use modint::StaticModint;
use union_find_with_potential::UnionFindWithPotential;

type Mint = StaticModint<998244353>;

query::define_query! {
    Query {
        0 => Query0(u: usize, v: usize, x: u64),
        1 => Query1(u: usize, v: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut uf = UnionFindWithPotential::<Mint>::new_default(n);

    for query in queries {
        match query {
            Query0(u, v, x) => {
                println!("{}", if uf.merge(u, v, Mint::new(x)).is_some() { 1 } else { 0 });
            }
            Query1(u, v) => {
                println!("{}", if uf.same(u, v) { uf.diff(u, v).to_string() } else { "-1".to_string() });
            }
        }
    }
}
