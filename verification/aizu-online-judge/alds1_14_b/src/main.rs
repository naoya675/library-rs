// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::{input, marker::Chars};

use rolling_hash::RollingHash;

fn main() {
    input! {
        t: Chars,
        p: Chars,
    }
    let mut rh = RollingHash::new();
    let ht = rh.build(&t);
    let hp = rh.build(&p);
    for i in 0.. {
        if i + p.len() > t.len() {
            break;
        }
        if rh.rolling_hash(&ht, i + 1, i + p.len()) == rh.rolling_hash(&hp, 1, p.len()) {
            println!("{}", i);
        }
    }
}
