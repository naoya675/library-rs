// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use proconio::input;

use lower_bound::LowerBound;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize, usize); q],
    }
    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let mut pos = vec![vec![]; sorted.len()];
    for (i, &a) in a.iter().enumerate() {
        let idx = sorted.lower_bound(&a);
        pos[idx].push(i);
    }

    for &(l, r, x) in &queries {
        let idx = sorted.lower_bound(&x);
        let res = if idx < sorted.len() && sorted[idx] == x {
            pos[idx].lower_bound(&r) - pos[idx].lower_bound(&l)
        } else {
            0
        };
        println!("{}", res);
    }
}
