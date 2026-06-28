// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use std::cell::{Cell, RefCell};

use proconio::input;

use fenwick_tree::FenwickTree;
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
    for &(l, r) in &lr {
        mo.add_query(l, r);
    }

    let ft = RefCell::new(FenwickTree::<i64>::new(x.len()));
    let inv = Cell::new(0);
    let mut res = vec![0; q];
    mo.run_queries(
        |i| {
            inv.set(inv.get() + ft.borrow().sum(0, a[i]));
            ft.borrow_mut().add(a[i], 1);
        },
        |i| {
            inv.set(inv.get() + ft.borrow().sum(a[i] + 1, x.len()));
            ft.borrow_mut().add(a[i], 1);
        },
        |i| {
            ft.borrow_mut().add(a[i], -1);
            inv.set(inv.get() - ft.borrow().sum(0, a[i]));
        },
        |i| {
            ft.borrow_mut().add(a[i], -1);
            inv.set(inv.get() - ft.borrow().sum(a[i] + 1, x.len()));
        },
        |idx| {
            res[idx] = inv.get();
        },
    );
    for i in 0..q {
        println!("{}", res[i]);
    }
}
