// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use fast_io::{Output, define_query, input, output};

use fenwick_tree::FenwickTree;

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

    let t = t.iter().map(|&t| (t as usize - '0' as usize) as i64).collect::<Vec<_>>();
    let mut ft = FenwickTree::from_slice(&t);

    for query in queries {
        match query {
            Query0(k) => {
                if ft.sum(k, k + 1) == 0 {
                    ft.add(k, 1);
                }
            }
            Query1(k) => {
                if ft.sum(k, k + 1) > 0 {
                    ft.add(k, -1);
                }
            }
            Query2(k) => {
                output!(out, ft.sum(k, k + 1));
            }
            Query3(k) => {
                if ft.sum(k, n) > 0 {
                    let cnt = ft.sum(0, k);
                    output!(out, ft.upper_bound(cnt));
                } else {
                    output!(out, -1);
                }
            }
            Query4(k) => {
                let cnt = ft.sum(0, k + 1);
                if cnt > 0 {
                    output!(out, ft.lower_bound(cnt));
                } else {
                    output!(out, -1);
                }
            }
        }
    }
}
