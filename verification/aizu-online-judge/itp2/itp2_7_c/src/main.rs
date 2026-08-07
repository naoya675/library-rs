// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_7_C

use fast_io::{Output, define_query, input, output};

use treap::Treap;

define_query! {
    Query {
        0 => Query0(x: i64),
        1 => Query1(x: i64),
        2 => Query2(x: i64),
        3 => Query3(l: i64, r: i64), // dump [l, r]
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut treap = Treap::new();

    for query in queries {
        match query {
            Query0(x) => {
                treap.insert(x);
                output!(out, treap.len());
            }
            Query1(x) => {
                output!(out, if treap.contains(&x) { 1 } else { 0 });
            }
            Query2(x) => {
                treap.remove(&x);
            }
            Query3(l, r) => {
                let lo = treap.lower_bound(&l);
                let hi = treap.upper_bound(&r);
                for k in lo..hi {
                    output!(out, treap.kth(k));
                }
            }
        }
    }
}
