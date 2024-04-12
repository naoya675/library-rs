// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A

use proconio::input;

use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
        qxy: [(usize, usize, usize); q],
    }
    let mut uf = UnionFind::new(n);
    for (query, x, y) in qxy {
        match query {
            0 => {
                uf.merge(x, y);
            }
            1 => {
                println!("{}", if uf.same(x, y) { 1 } else { 0 });
            }
            _ => unreachable!(),
        }
    }
}
