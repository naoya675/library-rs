// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_D

use proconio::{input, marker::Chars};

use aho_corasick::AhoCorasick;

fn main() {
    input! {
        t: Chars,
        q: usize,
        p: [Chars; q],
    }
    let mut ac = AhoCorasick::new(75, '0'); // '0'-'9', 'A'-'Z', 'a'-'z'
    for i in 0..q {
        ac.insert(&p[i]);
    }
    ac.build(true);

    let res = ac.matches(&t);
    for i in 0..q {
        println!("{}", if res[i] == 0 { 0 } else { 1 });
    }
}
