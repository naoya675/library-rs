// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use proconio::input;

use modint::Modint;
use union_find_with_potential::UnionFindWithPotential;

type Mint = Modint<998244353>;

query::define_query! {
    Query {
        0 => Query0(u: usize, v: usize, x: i64, y: i64, z: i64, w: i64),
        1 => Query1(u: usize, v: usize),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Matrix {
    mat: [[Mint; 2]; 2],
}

impl Matrix {
    fn new(mat: [[Mint; 2]; 2]) -> Self {
        Self { mat }
    }

    fn mul(lhs: Self, rhs: Self) -> Self {
        let mut ret = Self::new([[Mint::new(0); 2]; 2]);
        for i in 0..2 {
            for k in 0..2 {
                for j in 0..2 {
                    ret.mat[i][j] += lhs.mat[i][k] * rhs.mat[k][j];
                }
            }
        }
        ret
    }

    fn identity() -> Self {
        let mut mat = [[Mint::new(0); 2]; 2];
        for i in 0..2 {
            mat[i][i] = Mint::new(1);
        }
        Self { mat }
    }

    fn inv(a: Self) -> Self {
        Self {
            mat: [[a.mat[1][1], -a.mat[0][1]], [-a.mat[1][0], a.mat[0][0]]],
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut uf = UnionFindWithPotential::<Matrix>::new(n, Matrix::mul, Matrix::identity(), Matrix::inv);

    for query in queries {
        match query {
            Query0(u, v, x, y, z, w) => {
                let m = Matrix::new([[Mint::new(x), Mint::new(y)], [Mint::new(z), Mint::new(w)]]);
                println!("{}", if uf.merge(u, v, m).is_some() { 1 } else { 0 });
            }
            Query1(u, v) => {
                if uf.same(u, v) {
                    let m = uf.diff(u, v).mat;
                    println!("{} {} {} {}", m[0][0], m[0][1], m[1][0], m[1][1]);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
