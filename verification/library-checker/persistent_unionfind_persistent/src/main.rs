// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_unionfind

use proconio::input;

use persistent_union_find::PersistentUnionFind;

query::define_query! {
    Query {
        0 => Query0(k: i64, u: usize, v: usize),
        1 => Query1(k: i64, u: usize, v: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut versions = vec![];
    versions.push(PersistentUnionFind::new(n));
    for query in queries {
        match query {
            Query0(k, u, v) => {
                let next = versions[(k + 1) as usize].merge(u, v);
                versions.push(next);
            }
            Query1(k, u, v) => {
                println!("{}", if versions[(k + 1) as usize].same(u, v) { 1 } else { 0 });
                versions.push(versions[(k + 1) as usize].clone());
            }
        }
    }
}
