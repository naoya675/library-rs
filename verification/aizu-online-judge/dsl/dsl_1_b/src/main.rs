// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B

use fast_io::{Output, define_query, input, output};

use union_find_with_potential::UnionFindWithPotential;

define_query! {
    Query {
        0 => Query0(x: usize, y: usize, z: i64),
        1 => Query1(x: usize, y: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut uf = UnionFindWithPotential::new(n, |x, y| x + y, 0, |x| -x);

    for query in queries {
        match query {
            Query0(x, y, z) => {
                uf.merge(x, y, z);
            }
            Query1(x, y) => {
                if uf.same(x, y) {
                    output!(out, uf.diff(x, y));
                } else {
                    output!(out, "?");
                }
            }
        }
    }
}
