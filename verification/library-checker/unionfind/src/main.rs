// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;

use union_find::UnionFind;

query::define_query! {
    Query {
        0 => Query0(u: usize, v: usize),
        1 => Query1(u: usize, v: usize),
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
            Query0(u, v) => {
                uf.merge(u, v);
            }
            Query1(u, v) => {
                println!("{}", if uf.same(u, v) { 1 } else { 0 });
            }
        }
    }
}
