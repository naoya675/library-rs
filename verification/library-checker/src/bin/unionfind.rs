// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;

use data_structure::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
        tuv: [(usize, usize, usize); q],
    }
    let mut uf = UnionFind::new(n);
    for (t, u, v) in tuv {
        match t {
            0 => {
                uf.merge(u, v);
            }
            1 => {
                println!("{}", if uf.same(u, v) { 1 } else { 0 });
            }
            _ => unreachable!(),
        }
    }
}
