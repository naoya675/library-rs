// verification-helper: PROBLEM https://judge.yosupo.jp/problem/minimum_spanning_tree

use std::collections::HashMap;

use proconio::input;

use itertools::Join;
use kruskal::minimum_spanning_tree;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, i64); m],
    }
    let mut index: HashMap<(usize, usize, i64), usize> = HashMap::new();
    for (i, &e) in abc.iter().enumerate() {
        *index.entry(e).or_default() = i;
    }

    let (x, mst) = minimum_spanning_tree(n, &mut abc);
    let used = mst.iter().map(|e| index.get(e).unwrap()).collect::<Vec<_>>();

    println!("{}", x);
    println!("{}", used.iter().join(" "));
}
