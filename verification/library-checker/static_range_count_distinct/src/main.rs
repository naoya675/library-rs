// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_count_distinct

use std::cell::{Cell, RefCell};

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
    let mut x = a.clone();
    x.sort();
    x.dedup();
    let a: Vec<usize> = a.iter().map(|&a| x.lower_bound(&a)).collect();

    let mut mo = Mo::new(n, q);
    let mut qidx = vec![];
    for (i, &(l, r)) in lr.iter().enumerate() {
        if l < r {
            mo.add_query(l, r);
            qidx.push(i);
        }
    }
    let freq = RefCell::new(vec![0; x.len()]);
    let cnt = Cell::new(0);
    let mut res = vec![0; q];
    mo.run_queries_simple(
        |i| {
            if freq.borrow()[a[i]] == 0 {
                cnt.set(cnt.get() + 1);
            }
            freq.borrow_mut()[a[i]] += 1;
        },
        |i| {
            freq.borrow_mut()[a[i]] -= 1;
            if freq.borrow()[a[i]] == 0 {
                cnt.set(cnt.get() - 1);
            }
        },
        |idx| {
            res[qidx[idx]] = cnt.get();
        },
    );
    for i in 0..q {
        println!("{}", res[i]);
    }
}
