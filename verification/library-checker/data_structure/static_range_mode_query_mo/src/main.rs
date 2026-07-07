// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_mode_query

use std::cell::{Cell, RefCell};
use std::collections::HashSet;

use proconio::input;

use lower_bound::LowerBound;
use mo::Mo;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let a: Vec<usize> = a.iter().map(|v| sorted.lower_bound(v)).collect();

    let mut mo = Mo::new(n, q);
    for &(l, r) in &lr {
        mo.add_query(l, r);
    }
    let cnt = RefCell::new(vec![0; sorted.len()]);
    let cnt_vec = RefCell::new(vec![HashSet::new(); n + 1]);
    let best_cnt = Cell::new(0);
    let mut res = vec![(0, 0); q];
    mo.run_queries_simple(
        |i| {
            let c = a[i];
            let mut cnt = cnt.borrow_mut();
            let mut cnt_vec = cnt_vec.borrow_mut();
            if cnt[c] > 0 {
                cnt_vec[cnt[c]].remove(&c);
            }
            cnt[c] = cnt[c] + 1;
            cnt_vec[cnt[c]].insert(c);
            if best_cnt.get() < cnt[c] {
                best_cnt.set(cnt[c]);
            }
        },
        |i| {
            let c = a[i];
            let mut cnt = cnt.borrow_mut();
            let mut cnt_vec = cnt_vec.borrow_mut();
            let old = cnt[c];
            cnt_vec[old].remove(&c);
            if best_cnt.get() == cnt[c] && cnt_vec[cnt[c]].is_empty() {
                best_cnt.set(cnt[c] - 1);
            }
            cnt[c] = cnt[c] - 1;
            if cnt[c] > 0 {
                cnt_vec[cnt[c]].insert(c);
            }
        },
        |qid| {
            let cnt = cnt.borrow_mut();
            let cnt_vec = cnt_vec.borrow();
            if let Some(&c) = cnt_vec[best_cnt.get()].iter().next() {
                res[qid] = (sorted[c], cnt[c]);
            }
        },
    );

    for i in 0..q {
        println!("{} {}", res[i].0, res[i].1);
    }
}
