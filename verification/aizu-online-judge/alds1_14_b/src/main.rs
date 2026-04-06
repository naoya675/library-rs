// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::{input, marker::Chars};

use modint_mersenne61::ModintMersenne61;
use rolling_hash::RollingHash;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    let mut rh = RollingHash::<ModintMersenne61>::new(ModintMersenne61::rand());
    let mut ht = rh.build_segment_tree(&t);
    let mut hp = rh.build_segment_tree(&p);

    for i in 0.. {
        if i + p.len() > t.len() {
            break;
        }
        if ht.prod(i, i + p.len()) == hp.prod(0, p.len()) {
            println!("{}", i);
        }
    }
}
