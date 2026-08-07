// verification-helper: PROBLEM https://judge.yosupo.jp/problem/double_ended_priority_queue

use fast_io::{Output, define_query, input, output};

use treap_map::TreapMap;

define_query! {
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
    let mut out = Output::new();

    let mut treap = TreapMap::<i64, usize>::new();
    for s in s {
        *treap.entry(s).or_insert(0) += 1;
    }

    for query in queries {
        match query {
            Query0(x) => {
                *treap.entry(x).or_insert(0) += 1;
            }
            Query1() => {
                let min = *treap.min().unwrap().0;
                let c = treap.get_mut(&min).unwrap();
                *c -= 1;
                if *c == 0 {
                    treap.remove(&min);
                }
                output!(out, min);
            }
            Query2() => {
                let max = *treap.max().unwrap().0;
                let c = treap.get_mut(&max).unwrap();
                *c -= 1;
                if *c == 0 {
                    treap.remove(&max);
                }
                output!(out, max);
            }
        }
    }
}
