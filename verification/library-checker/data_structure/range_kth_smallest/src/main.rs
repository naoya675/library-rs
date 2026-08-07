// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest

use fast_io::{Output, input, output};

use persistent_segment_tree::PersistentSegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize, usize); q],
    }
    let mut out = Output::new();

    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let mut versions = vec![PersistentSegmentTree::new(sorted.len(), |x, y| x + y, 0)];
    for a in a {
        let rank = sorted.partition_point(|&x| x < a);
        let next = versions.last().unwrap().apply(rank, 1);
        versions.push(next);
    }

    for (l, r, k) in queries {
        let mut hi = sorted.len();
        let mut lo = 0;
        while lo < hi {
            let mi = (lo + hi) / 2;
            let cnt = versions[r].prod(0, mi + 1) - versions[l].prod(0, mi + 1);
            if cnt > k {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        output!(out, sorted[hi]);
    }
}
