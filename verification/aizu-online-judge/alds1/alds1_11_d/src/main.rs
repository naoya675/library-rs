// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_D

use proconio::input;

use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(usize, usize); m],
        q: usize,
        queries: [(usize, usize); q],
    }
    let mut uf = UnionFind::new(n);
    for &(s, t) in &st {
        uf.merge(s, t);
    }

    for &(s, t) in &queries {
        println!("{}", if uf.same(s, t) { "yes" } else { "no" })
    }
}
