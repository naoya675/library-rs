// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_unionfind

use fast_io::{Output, define_query, input, output};

use persistent_union_find::PersistentUnionFind;

define_query! {
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
    let mut out = Output::new();

    let mut versions = vec![PersistentUnionFind::new(n)];

    for query in queries {
        match query {
            Query0(k, u, v) => {
                let next = versions[(k + 1) as usize].merge(u, v);
                versions.push(next);
            }
            Query1(k, u, v) => {
                output!(out, if versions[(k + 1) as usize].same(u, v) { 1 } else { 0 });
                versions.push(versions[(k + 1) as usize].clone());
            }
        }
    }
}
