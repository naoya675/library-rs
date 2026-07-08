// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_mode_query

use proconio::input;

use range_mode_query::RangeModeQuery;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let rmq = RangeModeQuery::new(&a);

    for &(l, r) in &lr {
        let (mode, freq) = rmq.query(l, r);
        println!("{} {}", mode, freq);
    }
}
