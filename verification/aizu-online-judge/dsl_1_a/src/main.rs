// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A

use proconio::input;

use union_find::UnionFind;

query::define_query! {
    Query {
        0 => Query0(x: usize, y: usize),
        1 => Query1(x: usize, y: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut uf = UnionFind::new(n);

    for query in queries {
        match query {
            Query0(x, y) => {
                uf.merge(x, y);
            }
            Query1(x, y) => {
                println!("{}", if uf.same(x, y) { 1 } else { 0 });
            }
        }
    }
}
