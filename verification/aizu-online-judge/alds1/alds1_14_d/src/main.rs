// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_D

use fast_io::{Output, input, output};

use aho_corasick::AhoCorasick;

fn main() {
    input! {
        t: Chars,
        q: usize,
        p: [Chars; q],
    }
    let mut out = Output::new();

    let mut ac = AhoCorasick::new(75, '0'); // '0'-'9', 'A'-'Z', 'a'-'z'
    for i in 0..q {
        ac.insert(&p[i]);
    }
    ac.build(true);

    let res = ac.matches(&t);

    for res in res {
        output!(out, if res == 0 { 0 } else { 1 });
    }
}
