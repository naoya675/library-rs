// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use fast_io::{Output, input, output};

use lower_bound::LowerBound;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize, i64); q],
    }
    let mut out = Output::new();

    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let mut pos = vec![vec![]; sorted.len()];
    for (i, &a) in a.iter().enumerate() {
        let rank = sorted.lower_bound(&a);
        pos[rank].push(i);
    }

    for (l, r, x) in queries {
        let rank = sorted.lower_bound(&x);
        let res = if rank < sorted.len() && sorted[rank] == x {
            pos[rank].lower_bound(&r) - pos[rank].lower_bound(&l)
        } else {
            0
        };
        output!(out, res);
    }
}
