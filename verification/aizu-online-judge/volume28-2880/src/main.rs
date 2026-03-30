// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2880

use proconio::input;

use interval_set::IntervalSet;

fn main() {
    input! {
        _n: usize,
        m: usize,
        q: usize,
    }
    let mut query = vec![];
    for i in 0..m {
        input! {
            d: usize,
            a: usize,
            b: usize,
        }
        query.push((d, 1, i, a * 2, b * 2 + 1));
    }
    for i in 0..q {
        input! {
            e: usize,
            s: usize,
            t: usize,
        }
        query.push((e, 0, i, s * 2, t * 2));
    }
    query.sort();
    let mut set = IntervalSet::<usize, usize>::new(0);
    let mut res = vec![false; q];
    for &(_, q, i, s, t) in &query {
        match q {
            0 => {
                res[i] = s >= t || set.same(s, t);
            }
            1 => {
                set.insert(s, t);
            }
            _ => unreachable!(),
        }
    }
    for i in 0..q {
        println!("{}", if res[i] { "Yes" } else { "No" });
    }
}
