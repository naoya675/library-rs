// verification-helper: PROBLEM https://judge.yosupo.jp/problem/double_ended_priority_queue

use proconio::input;

use treap::Treap;

query::define_query! {
    Query {
        0 => Query0(x: i64),
        1 => Query1(),
        2 => Query2(),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [i64; n],
        queries: [Query; q],
    }
    let mut treap = Treap::<i64>::new();
    for &s in &s {
        treap.insert(s);
    }
    for query in queries {
        match query {
            Query0(x) => {
                treap.insert(x);
            }
            Query1() => {
                let min = treap.min().unwrap();
                treap.remove(min);
                println!("{}", min);
            }
            Query2() => {
                let max = treap.max().unwrap();
                treap.remove(max);
                println!("{}", max);
            }
        }
    }
}
