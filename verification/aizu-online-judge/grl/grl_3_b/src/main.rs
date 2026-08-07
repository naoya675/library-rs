// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_B

use fast_io::{Output, input, output};

use low_link::LowLink;

fn main() {
    input! {
        v: usize,
        e: usize,
        st: [(usize, usize); e],
    }
    let mut out = Output::new();

    let mut ll = LowLink::new(v);
    for (s, t) in st {
        ll.add_edge(s, t);
    }
    ll.build();

    let mut bridges = ll.bridge().iter().map(|&(s, t)| if s < t { (s, t) } else { (t, s) }).collect::<Vec<_>>();
    bridges.sort();

    for (s, t) in bridges {
        output!(out, s, t);
    }
}
