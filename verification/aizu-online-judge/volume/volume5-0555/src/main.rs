// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0555

use fast_io::{Output, input, output};

use kmp::Kmp;

fn main() {
    input! {
        p: Chars,
        n: usize,
        s: [Chars; n],
    }
    let mut out = Output::new();

    let kmp = Kmp::new(&p);

    let mut res = 0;
    for s in &s {
        let ring = [s.as_slice(), s.as_slice()].concat();
        if !kmp.pattern_matching(&ring).is_empty() {
            res += 1;
        }
    }

    output!(out, res);
}
