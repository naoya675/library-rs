// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use proconio::input;

use modint::StaticModint;
use union_find_with_potential::UnionFindWithPotential;

type Mint = StaticModint<998244353>;

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

    pub fn add(a: Matrix, b: Matrix) -> Self {
        Self {
            mat: (a.mat.0 + b.mat.0, a.mat.1 + b.mat.1, a.mat.2 + b.mat.2, a.mat.3 + b.mat.3),
        }
    }

    pub fn mul(a: Matrix, b: Matrix) -> Self {
        Self {
            mat: (
                a.mat.0 * b.mat.0 + a.mat.1 * b.mat.2,
                a.mat.0 * b.mat.1 + a.mat.1 * b.mat.3,
                a.mat.2 * b.mat.0 + a.mat.3 * b.mat.2,
                a.mat.2 * b.mat.1 + a.mat.3 * b.mat.3,
            ),
        }
    }

    pub fn inv(a: Matrix) -> Self {
        Self {
            mat: (a.mat.3, -a.mat.1, -a.mat.2, a.mat.0),
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFindWithPotential::<Matrix>::new(n, |a, b| Matrix::mul(a, b), Matrix::new(1, 0, 0, 1), |a| Matrix::inv(a));
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { u: usize, v: usize, x: u64, y: u64, z: u64, w: u64 }
                println!("{}", if let Some(_) = uf.merge(u, v, Matrix::new(x, y, z, w)) { 1 } else { 0 });
            }
            1 => {
                input! { u: usize, v: usize, }
                if uf.same(u, v) {
                    let (x, y, z, w) = uf.diff(u, v).mat;
                    println!("{} {} {} {}", x, y, z, w);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
