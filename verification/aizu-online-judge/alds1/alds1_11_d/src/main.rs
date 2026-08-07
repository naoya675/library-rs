// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_D

use fast_io::{Output, input, output};

use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(usize, usize); m],
        q: usize,
        queries: [(usize, usize); q],
    }
    let mut out = Output::new();

    let mut uf = UnionFind::new(n);
    for (s, t) in st {
        uf.merge(s, t);
    }

    for (s, t) in queries {
        output!(out, if uf.same(s, t) { "yes" } else { "no" });
    }
}
