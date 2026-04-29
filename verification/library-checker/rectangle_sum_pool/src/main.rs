// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum

use proconio::input;

use lower_bound::LowerBound;
use persistent_segment_tree_pool::PersistentSegmentTreePool;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut xyw: [(i64, i64, i64); n],
        queries: [(i64, i64, i64, i64); q],
    }
    xyw.sort_by_key(|&(x, _, _)| x);
    let sorted_x = xyw.iter().map(|&(x, _, _)| x).collect::<Vec<_>>();
    let mut sorted_y = xyw.iter().map(|&(_, y, _)| y).collect::<Vec<_>>();
    sorted_y.sort();
    sorted_y.dedup();
    let mut pool = PersistentSegmentTreePool::<i64>::new(sorted_y.len(), |x, y| x + y, 0);
    let mut versions = vec![pool.empty()];
    for &(_, y, w) in &xyw {
        let rank = sorted_y.lower_bound(&y);
        let v = versions.last().copied().unwrap();
        let next = pool.apply(v, rank, w);
        versions.push(next);
    }

    for &(l, d, r, u) in &queries {
        let l = sorted_x.lower_bound(&l);
        let r = sorted_x.lower_bound(&r);
        let d = sorted_y.lower_bound(&d);
        let u = sorted_y.lower_bound(&u);
        println!("{}", pool.prod(versions[r], d, u) - pool.prod(versions[l], d, u));
    }
}
