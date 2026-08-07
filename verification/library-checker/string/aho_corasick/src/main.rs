// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aho_corasick

use fast_io::{Output, input, output};

use aho_corasick::AhoCorasick;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut out = Output::new();

    let mut ac = AhoCorasick::new(26, 'a');
    for s in &s {
        ac.insert(s);
    }
    ac.build(true);

    let mut index = vec![None; s.iter().map(|s| s.len()).sum::<usize>() + 1];
    index[0] = Some(0);
    let mut ps = vec![(0, 0)];
    let mut v = vec![];
    for s in &s {
        let mut now = 0;
        for &c in s {
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

    output!(out, ps.len());
    for &(p, s) in &ps[1..] {
        let p = index[p].unwrap();
        let s = index[s].unwrap();
        output!(out, p, s);
    }
    out.println_iter(v.iter().map(|&v| index[v].unwrap()), " ");
}
