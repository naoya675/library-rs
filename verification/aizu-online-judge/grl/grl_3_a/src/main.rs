// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_A

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

    let mut aps = ll.articulation().iter().map(|&v| v).collect::<Vec<_>>();
    aps.sort();

    for v in aps {
        output!(out, v);
    }
}
