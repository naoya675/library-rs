// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rooted_tree_isomorphism_classification

use std::collections::HashMap;

use fast_io::{Output, input, output};

use modint_mersenne61::ModintMersenne61;
use tree_isomorphism::hash_h;

type Mint = ModintMersenne61;

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    }
    let mut out = Output::new();

    let mut tree = vec![vec![]; n];
    for (i, &p) in p.iter().enumerate() {
        tree[p].push(i + 1);
        tree[i + 1].push(p);
    }

    let mut id = HashMap::new();
    let r: Vec<Mint> = (0..n).map(|_| Mint::rand()).collect();
    let hs = hash_h(&tree, 0, &r);
    let res = hs
        .iter()
        .map(|h| {
            let next = id.len();
            *id.entry(h.value()).or_insert(next)
        })
        .collect::<Vec<_>>();

    output!(out, id.len());
    out.println_iter(&res, " ");
}
