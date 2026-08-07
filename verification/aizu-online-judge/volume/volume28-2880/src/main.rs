// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2880

use fast_io::{Output, input, output};

use interval_set::IntervalSet;

fn main() {
    input! {
        _n: usize,
        m: usize,
        q: usize,
        dab: [(usize, usize, usize); m],
        est: [(usize, usize, usize); q],
    }
    let mut out = Output::new();

    let mut query = vec![];
    for (i, &(d, a, b)) in dab.iter().enumerate() {
        query.push((d, 1, i, a * 2, b * 2 + 1));
    }
    for (i, &(e, s, t)) in est.iter().enumerate() {
        query.push((e, 0, i, s * 2, t * 2));
    }
    query.sort();

    let mut set = IntervalSet::<usize, usize>::new(0);
    let mut res = vec![false; q];
    for &(_, q, i, s, t) in &query {
        match q {
            0 => {
                res[i] = set.same(s, t) || s >= t;
            }
            1 => {
                set.insert(s, t);
            }
            _ => unreachable!(),
        }
    }

    for &res in &res {
        output!(out, if res { "Yes" } else { "No" });
    }
}
