// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use fast_io::{Output, input, output};

use modint_mersenne61::ModintMersenne61;
use rolling_hash::RollingHash;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    let mut out = Output::new();

    let mut rh = RollingHash::new(ModintMersenne61::rand());
    let ht = rh.build_segment_tree(&t);
    let hp = rh.build_segment_tree(&p);

    for i in 0.. {
        if i + p.len() > t.len() {
            break;
        }
        if ht.prod(i, i + p.len()) == hp.prod(0, p.len()) {
            output!(out, i);
        }
    }
}
