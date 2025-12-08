// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use proconio::input;

use modint::StaticModint;
use union_find_with_potential::UnionFindWithPotential;

type Mint = StaticModint<998244353>;

query::define_query! {
    Query {
        0 => Query0(u: usize, v: usize, x: u64, y: u64, z: u64, w: u64),
        1 => Query1(u: usize, v: usize),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix {
    mat: (Mint, Mint, Mint, Mint),
}

impl Matrix {
    pub fn new(x: u64, y: u64, z: u64, w: u64) -> Self {
        Self {
            mat: (Mint::new(x), Mint::new(y), Mint::new(z), Mint::new(w)),
        }
    }

    pub fn inv(x: Matrix) -> Self {
        Self {
            mat: (x.mat.3, -x.mat.1, -x.mat.2, x.mat.0),
        }
    }

    pub fn add(x: Matrix, y: Matrix) -> Self {
        Self {
            mat: (x.mat.0 + y.mat.0, x.mat.1 + y.mat.1, x.mat.2 + y.mat.2, x.mat.3 + y.mat.3),
        }
    }

    pub fn mul(x: Matrix, y: Matrix) -> Self {
        Self {
            mat: (
                x.mat.0 * y.mat.0 + x.mat.1 * y.mat.2,
                x.mat.0 * y.mat.1 + x.mat.1 * y.mat.3,
                x.mat.2 * y.mat.0 + x.mat.3 * y.mat.2,
                x.mat.2 * y.mat.1 + x.mat.3 * y.mat.3,
            ),
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut uf = UnionFindWithPotential::<Matrix>::new(n, |x, y| Matrix::mul(x, y), Matrix::new(1, 0, 0, 1), |x| Matrix::inv(x));

    for query in queries {
        match query {
            Query0(u, v, x, y, z, w) => {
                println!("{}", if uf.merge(u, v, Matrix::new(x, y, z, w)).is_some() { 1 } else { 0 });
            }
            Query1(u, v) => {
                if uf.same(u, v) {
                    let (x, y, z, w) = uf.diff(u, v).mat;
                    println!("{} {} {} {}", x, y, z, w);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
