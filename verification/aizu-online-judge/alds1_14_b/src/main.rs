// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::{input, marker::Chars};

use mersenne_modint::MersenneModint;
use rolling_hash_segment_tree::RollingHash;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    let mut rh = RollingHash::<MersenneModint>::new(MersenneModint::rand());
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
