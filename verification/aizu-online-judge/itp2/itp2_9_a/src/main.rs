// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_9_A

use fast_io::{Output, input, output};

use treap::Treap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
    }
    let mut out = Output::new();

    let mut treap = Treap::new();
    for x in a {
        treap.insert(x);
    }
    for x in b {
        treap.insert(x);
    }

    for &v in treap.iter() {
        output!(out, v);
    }
}
