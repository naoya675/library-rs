// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use fast_io::{Output, define_query, input, output};

use union_find::UnionFind;

define_query! {
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
    let mut out = Output::new();

    let mut uf = UnionFind::new(n);

    for query in queries {
        match query {
            Query0(u, v) => {
                uf.merge(u, v);
            }
            Query1(u, v) => {
                output!(out, if uf.same(u, v) { 1 } else { 0 });
            }
        }
    }
}
