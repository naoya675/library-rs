// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_mode_query

use std::cell::{Cell, RefCell};

use proconio::input;

use lower_bound::LowerBound;
use mo_with_rollback::MoWithRollback;

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

    let mut mo = MoWithRollback::new(n, q);
    for &(l, r) in &lr {
        mo.add_query(l, r);
    }
    let cnt = RefCell::new(vec![0; a.len()]);
    let best_val = Cell::new(0);
    let best_val_snap = Cell::new(0);
    let best_cnt = Cell::new(0);
    let best_cnt_snap = Cell::new(0);
    let history_snap = Cell::new(0);
    let history: RefCell<Vec<usize>> = RefCell::new(vec![]);
    let mut res = vec![(0, 0); q];
    mo.run_queries(
        |i| {
            let c = a[i];
            let mut cnt = cnt.borrow_mut();
            cnt[c] += 1;
            if cnt[c] > best_cnt.get() {
                best_val.set(sorted[c]);
                best_cnt.set(cnt[c]);
            }
            history.borrow_mut().push(c);
        },
        || {
            cnt.borrow_mut().fill(0);
            best_val.set(0);
            best_cnt.set(0);
            history.borrow_mut().clear();
        },
        || {
            best_val_snap.set(best_val.get());
            best_cnt_snap.set(best_cnt.get());
            history_snap.set(history.borrow().len());
        },
        || {
            let target = history_snap.get();
            let mut h = history.borrow_mut();
            let mut cnt = cnt.borrow_mut();
            while h.len() > target {
                let c = h.pop().unwrap();
                cnt[c] -= 1;
            }
            best_val.set(best_val_snap.get());
            best_cnt.set(best_cnt_snap.get());
        },
        |qid| {
            res[qid] = (best_val.get(), best_cnt.get());
        },
    );

    for i in 0..q {
        println!("{} {}", res[i].0, res[i].1);
    }
}
