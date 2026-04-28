// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use proconio::input;

use persistent_segment_tree::PersistentSegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize, i64); q],
    }

    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let mut versions = vec![];
    versions.push(PersistentSegmentTree::<i64>::new(sorted.len() + 1, |x, y| x + y, 0));
    for &a in &a {
        let rank = sorted.partition_point(|&x| x < a);
        let next = versions.last().unwrap().apply(rank, 1);
        versions.push(next);
    }

    for &(l, r, x) in &queries {
        let rank = sorted.partition_point(|&y| y < x);
        let res = if rank < sorted.len() && sorted[rank] == x {
            versions[r].prod(rank, rank + 1) - versions[l].prod(rank, rank + 1)
        } else {
            0
        };
        println!("{}", res);
    }
}
