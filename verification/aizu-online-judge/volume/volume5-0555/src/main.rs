// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0555

use proconio::{input, marker::Chars};

use kmp::Kmp;

fn main() {
    input! {
        p: Chars,
        n: usize,
        s: [Chars; n],
    }
    let kmp = Kmp::new(&p);

    let mut res = 0;
    for i in 0..n {
        let ring = [s[i].as_slice(), s[i].as_slice()].concat();
        if kmp.pattern_matching(&ring).len() > 0 {
            res += 1;
        }
    }

    println!("{}", res);
}
