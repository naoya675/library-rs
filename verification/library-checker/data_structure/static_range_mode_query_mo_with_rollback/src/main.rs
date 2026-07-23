// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_mode_query

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

    struct State {
        cnt: Vec<usize>,
        best: (usize, usize),
        best_snap: (usize, usize),
        history: Vec<usize>,
        history_snap: usize,
    }

    let mut state = State {
        cnt: vec![0; sorted.len()],
        best: (0, 0),
        best_snap: (0, 0),
        history: vec![],
        history_snap: 0,
    };
    let mut res = vec![(0, 0); q];
    mo.run_queries(
        &mut state,
        |st, i| {
            let c = a[i];
            st.cnt[c] += 1;
            if st.cnt[c] > st.best.1 {
                st.best = (sorted[c], st.cnt[c]);
            }
            st.history.push(c);
        },
        |st| {
            st.cnt.fill(0);
            st.best = (0, 0);
            st.history.clear();
        },
        |st| {
            st.best_snap = st.best;
            st.history_snap = st.history.len();
        },
        |st| {
            while st.history.len() > st.history_snap {
                let c = st.history.pop().unwrap();
                st.cnt[c] -= 1;
            }
            st.best = st.best_snap;
        },
        |st, qid| {
            res[qid] = st.best;
        },
    );

    for i in 0..q {
        println!("{} {}", res[i].0, res[i].1);
    }
}
