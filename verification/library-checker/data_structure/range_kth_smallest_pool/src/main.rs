// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest

use proconio::input;

use persistent_segment_tree_pool::PersistentSegmentTreePool;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize, usize); q],
    }
    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let mut versions: Vec<Option<u32>> = vec![None];
    let mut pool = PersistentSegmentTreePool::<usize>::new(sorted.len(), |x, y| x + y, 0);
    for &a in &a {
        let v = versions.last().copied().unwrap();
        let rank = sorted.partition_point(|&x| x < a);
        let next = pool.apply(v, rank, 1);
        versions.push(next);
    }

    for &(l, r, k) in &queries {
        let mut hi = sorted.len();
        let mut lo = 0;
        while lo < hi {
            let mi = (lo + hi) / 2;
            let cnt = pool.prod(versions[r], 0, mi + 1) - pool.prod(versions[l], 0, mi + 1);
            if cnt > k {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        println!("{}", sorted[hi]);
    }
}
