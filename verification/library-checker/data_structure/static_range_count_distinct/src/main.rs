// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_count_distinct

use fast_io::{Output, input, output};

use lower_bound::LowerBound;
use mo::Mo;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let mut out = Output::new();

    let mut x = a.clone();
    x.sort();
    x.dedup();
    let a: Vec<usize> = a.iter().map(|&a| x.lower_bound(&a)).collect();
    let mut mo = Mo::new(n, q);
    let mut qid = vec![];
    for (i, &(l, r)) in lr.iter().enumerate() {
        if l < r {
            mo.add_query(l, r);
            qid.push(i);
        }
    }

    struct State {
        freq: Vec<usize>,
        cnt: usize,
    }

    let mut state = State {
        freq: vec![0; x.len()],
        cnt: 0,
    };
    let mut res = vec![0; q];
    mo.run_queries_simple(
        &mut state,
        |state, i| {
            if state.freq[a[i]] == 0 {
                state.cnt += 1;
            }
            state.freq[a[i]] += 1;
        },
        |state, i| {
            state.freq[a[i]] -= 1;
            if state.freq[a[i]] == 0 {
                state.cnt -= 1;
            }
        },
        |state, idx| {
            res[qid[idx]] = state.cnt;
        },
    );

    for res in res {
        output!(out, res);
    }
}
