// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear

use proconio::input;

use floor_sum::floor_sum;

fn main() {
    input! {
        t: usize,
        nmab: [(i64, i64, i64, i64); t],
    }

    for &(n, m, a, b) in &nmab {
        println!("{}", floor_sum(n, m, a, b));
    }
}
