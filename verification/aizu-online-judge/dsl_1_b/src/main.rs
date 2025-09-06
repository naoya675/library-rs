// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B

use proconio::input;

use union_find_with_potential::UnionFindWithPotential;

query::define_query! {
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
    let mut uf = UnionFindWithPotential::<i64>::new_default(n);

    for query in queries {
        match query {
            Query0(x, y, z) => {
                uf.merge(x, y, z);
            }
            Query1(x, y) => {
                println!("{}", if uf.same(x, y) { uf.diff(x, y).to_string() } else { "?".to_string() });
            }
        }
    }
}
