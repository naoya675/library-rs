// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_reverse_range_sum

use proconio::input;

use implicit_treap::ImplicitTreap;

query::define_query! {
    Query {
        0 => Query0(l: usize, r: usize),
        1 => Query1(l: usize, r: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [Query; q],
    }
    let mut treap = ImplicitTreap::<u64, ()>::new(|x, y| x + y, 0, |_, x| x, |_, _| (), ());
    for i in 0..n {
        treap.insert(i, a[i]);
    }

    for query in queries {
        match query {
            Query0(l, r) => treap.reverse(l, r),
            Query1(l, r) => println!("{}", treap.prod(l, r)),
        }
    }
}
