// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aho_corasick

use itertools::Itertools;
use proconio::{input, marker::Chars};

use aho_corasick::AhoCorasick;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut ac = AhoCorasick::new(26, 'a');
    for i in 0..n {
        ac.insert(&s[i]);
    }
    ac.build(true);

    let mut index = vec![None; s.iter().map(|s| s.len()).sum::<usize>() + 1];
    index[0] = Some(0);
    let mut ps = vec![(0, 0)];
    let mut v = vec![];
    for i in 0..n {
        let mut now = 0;
        for &c in &s[i] {
            let next = ac.goto(now, c);
            let fail = ac.fail(next);
            if index[next].is_none() {
                index[next] = Some(ps.len());
                ps.push((now, fail));
            }
            now = next;
        }
        v.push(now);
    }

    println!("{}", ps.len());
    for &(p, s) in &ps[1..] {
        let p = index[p].unwrap();
        let s = index[s].unwrap();
        println!("{} {}", p, s);
    }
    println!("{}", v.iter().map(|&v| index[v].unwrap()).join(" "));
}
