// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rooted_tree_isomorphism_classification

use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

use modint_mersenne61::ModintMersenne61;
use tree_isomorphism::hash_h;

type Mint = ModintMersenne61;

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    }
    let mut tree = vec![vec![]; n];
    for (i, &p) in p.iter().enumerate() {
        tree[p].push(i + 1);
        tree[i + 1].push(p);
    }
    let mut id: HashMap<u64, usize> = HashMap::new();
    let r: Vec<Mint> = (0..n).map(|_| Mint::rand()).collect();
    let hs = hash_h(&tree, 0, &r);
    let res = hs
        .iter()
        .map(|h| {
            let next = id.len();
            *id.entry(h.value()).or_insert(next)
        })
        .collect::<Vec<_>>();
    println!("{}", id.len());
    println!("{}", res.iter().join(" "));
}
