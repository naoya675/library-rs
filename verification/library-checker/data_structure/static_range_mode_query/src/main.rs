// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_mode_query

use fast_io::{Output, input, output};

use range_mode_query::RangeModeQuery;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let mut out = Output::new();

    let rmq = RangeModeQuery::new(&a);

    for (l, r) in lr {
        let (mode, freq) = rmq.query(l, r);
        output!(out, mode, freq);
    }
}
