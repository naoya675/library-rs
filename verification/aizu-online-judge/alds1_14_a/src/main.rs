// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_A

use proconio::{input, marker::Chars};

use mersenne_modint::MersenneModint;
use rolling_hash::RollingHash;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    let mut rh = RollingHash::<MersenneModint>::new(MersenneModint::rand());
    let ht = rh.build(&t);
    let hp = rh.build(&p);

    for i in 0.. {
        if i + p.len() > t.len() {
            break;
        }
        if rh.rolling_hash(&ht, i, i + p.len()) == rh.rolling_hash(&hp, 0, p.len()) {
            println!("{}", i);
        }
    }
}
