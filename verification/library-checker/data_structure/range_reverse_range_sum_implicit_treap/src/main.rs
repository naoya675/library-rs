// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_reverse_range_sum

use fast_io::{Output, define_query, input, output};

use implicit_treap::ImplicitTreap;

define_query! {
    Query {
        0 => Query0(l: usize, r: usize),
        1 => Query1(l: usize, r: usize),
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

    let mut treap = ImplicitTreap::new(|x, y| x + y, 0, |_, x| x, |_, _| (), ());
    for (i, &a) in a.iter().enumerate() {
        treap.insert(i, a);
    }

    for query in queries {
        match query {
            Query0(l, r) => treap.reverse(l, r),
            Query1(l, r) => output!(out, treap.prod(l, r)),
        }
    }
}
