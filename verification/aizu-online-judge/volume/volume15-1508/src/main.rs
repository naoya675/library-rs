// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1508

use fast_io::{Output, define_query, input, output};

use implicit_treap::ImplicitTreap;

define_query! {
    Query {
        0 => Query0(l: usize, r: usize),
        1 => Query1(l: usize, r: usize),
        2 => Query2(p: usize, v: i64),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut treap = ImplicitTreap::from_slice(&a, |x, y| std::cmp::min(x, y), i64::MAX, |_, x| x, |_, _| (), ());

    for query in queries {
        match query {
            Query0(l, r) => {
                treap.rotate(l, r + 1, r - l);
            }
            Query1(l, r) => {
                output!(out, treap.prod(l, r + 1));
            }
            Query2(p, v) => {
                treap.set(p, v);
            }
        }
    }
}
