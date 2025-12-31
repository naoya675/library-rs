// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2880

use proconio::input;

use interval_set::IntervalSet;

fn main() {
    input! {
        _n: usize,
        m: usize,
        q: usize,
        dab: [(usize, usize, usize); m],
        est: [(usize, usize, usize); q],
    }
    let mut query = vec![];
    for (_, &(d, a, b)) in dab.iter().enumerate() {
        query.push((d * 2 + 1, 0, a * 2, b * 2 + 1));
    }
    for (i, &(e, s, t)) in est.iter().enumerate() {
        query.push((e * 2, i + 1, s * 2, t * 2));
    }
    query.sort();
    let mut set = IntervalSet::<usize, usize>::new(0);
    let mut res = vec![false; q + 1];
    for &(_, q, s, t) in &query {
        match q {
            0 => {
                set.insert(s, t);
            }
            _ => {
                res[q] = s >= t || set.same(s, t);
            }
        }
    }
    for i in 1..=q {
        println!("{}", if res[i] { "Yes" } else { "No" });
    }
}
