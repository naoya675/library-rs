// verification-helper: PROBLEM https://judge.yosupo.jp/problem/ordered_set

use fast_io::{Output, define_query, input, output};

use treap::Treap;

define_query! {
    Query {
        0 => Query0(x: usize),
        1 => Query1(x: usize),
        2 => Query2(x: usize),
        3 => Query3(x: usize),
        4 => Query4(x: usize),
        5 => Query5(x: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut treap = Treap::new();
    for x in a {
        treap.insert(x);
    }

    for query in queries {
        match query {
            Query0(x) => {
                treap.insert(x);
            }
            Query1(x) => {
                treap.remove(&x);
            }
            Query2(x) => {
                if x <= treap.len() {
                    output!(out, treap.kth(x - 1));
                } else {
                    output!(out, -1);
                }
            }
            Query3(x) => {
                output!(out, treap.upper_bound(&x));
            }
            Query4(x) => {
                if let Some(&y) = treap.predecessor(&x) {
                    output!(out, y);
                } else {
                    output!(out, -1);
                }
            }
            Query5(x) => {
                if let Some(&y) = treap.successor(&x) {
                    output!(out, y);
                } else {
                    output!(out, -1);
                }
            }
        }
    }
}
