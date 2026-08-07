// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_frequency

use std::collections::HashMap;

use fast_io::{Output, define_query, input, output};

use treap::Treap;

define_query! {
    Query {
        0 => Query0(k: usize, v: i64),
        1 => Query1(l: usize, r: usize, x: i64),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut pos = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        pos.entry(x).or_insert(Treap::new()).insert(i);
    }

    for query in queries {
        match query {
            Query0(k, v) => {
                pos.get_mut(&a[k]).unwrap().remove(&k);
                pos.entry(v).or_insert(Treap::new()).insert(k);
                a[k] = v;
            }
            Query1(l, r, x) => {
                let cnt = match pos.get(&x) {
                    Some(t) => t.lower_bound(&r) - t.lower_bound(&l),
                    None => 0,
                };
                output!(out, cnt);
            }
        }
    }
}
