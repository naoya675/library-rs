// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential

use fast_io::{Output, define_query, input, output};

use modint::Modint;
use union_find_with_potential::UnionFindWithPotential;

type Mint = Modint<998244353>;

define_query! {
    Query {
        0 => Query0(u: usize, v: usize, x: i64),
        1 => Query1(u: usize, v: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut uf = UnionFindWithPotential::new(n, |x, y| x + y, Mint::new(0), |x| -x);

    for query in queries {
        match query {
            Query0(u, v, x) => {
                output!(out, if uf.merge(u, v, Mint::new(x)).is_some() { 1 } else { 0 });
            }
            Query1(u, v) => {
                if uf.same(u, v) {
                    output!(out, uf.diff(u, v).value());
                } else {
                    output!(out, -1);
                }
            }
        }
    }
}
