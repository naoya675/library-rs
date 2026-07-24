// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_mode_query

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

    struct State {
        cnt: Vec<usize>,
        cnt_vec: Vec<HashSet<usize>>,
        best_cnt: usize,
    }

    let mut state = State {
        cnt: vec![0; sorted.len()],
        cnt_vec: vec![HashSet::new(); n + 1],
        best_cnt: 0,
    };
    let mut res = vec![(0, 0); q];
    mo.run_queries_simple(
        &mut state,
        |st, i| {
            let c = a[i];
            if st.cnt[c] > 0 {
                st.cnt_vec[st.cnt[c]].remove(&c);
            }
            st.cnt[c] += 1;
            st.cnt_vec[st.cnt[c]].insert(c);
            if st.best_cnt < st.cnt[c] {
                st.best_cnt = st.cnt[c];
            }
        },
        |st, i| {
            let c = a[i];
            let old = st.cnt[c];
            st.cnt_vec[old].remove(&c);
            if st.best_cnt == st.cnt[c] && st.cnt_vec[st.cnt[c]].is_empty() {
                st.best_cnt = st.cnt[c] - 1;
            }
            st.cnt[c] -= 1;
            if st.cnt[c] > 0 {
                st.cnt_vec[st.cnt[c]].insert(c);
            }
        },
        |st, qid| {
            if let Some(&c) = st.cnt_vec[st.best_cnt].iter().next() {
                res[qid] = (sorted[c], st.cnt[c]);
            }
        },
    );

    for i in 0..q {
        println!("{} {}", res[i].0, res[i].1);
    }
}
