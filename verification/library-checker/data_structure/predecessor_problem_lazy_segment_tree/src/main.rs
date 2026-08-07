// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use fast_io::{Output, define_query, input, output};

use lazy_segment_tree::LazySegmentTree;

define_query! {
    Query {
        0 => Query0(k: usize),
        1 => Query1(k: usize),
        2 => Query2(k: usize),
        3 => Query3(k: usize),
        4 => Query4(k: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        t: Chars,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let t = t.iter().map(|&t| t as usize - '0' as usize).collect::<Vec<_>>();
    let mut lst = LazySegmentTree::from_slice(&t, |x, y| x + y, 0, |_, x| x, |_, _| (), ());

    for query in queries {
        match query {
            Query0(k) => {
                lst.set(k, 1);
            }
            Query1(k) => {
                lst.set(k, 0);
            }
            Query2(k) => {
                output!(out, lst.get(k));
            }
            Query3(k) => {
                let r = lst.max_right(k, |x| x == 0);
                if r < n {
                    output!(out, r);
                } else {
                    output!(out, -1);
                }
            }
            Query4(k) => {
                let l = lst.min_left(k + 1, |x| x == 0);
                if l > 0 {
                    output!(out, l - 1);
                } else {
                    output!(out, -1);
                }
            }
        }
    }
}
