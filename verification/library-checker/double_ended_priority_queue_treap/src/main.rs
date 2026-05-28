// verification-helper: PROBLEM https://judge.yosupo.jp/problem/double_ended_priority_queue

use proconio::input;

use treap_map::TreapMap;

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
    let mut treap = TreapMap::<i64, usize>::new();
    for &s in &s {
        if let Some(c) = treap.get_mut(&s) {
            *c += 1;
        } else {
            treap.insert(s, 1);
        }
    }
    for query in queries {
        match query {
            Query0(x) => {
                if let Some(c) = treap.get_mut(&x) {
                    *c += 1;
                } else {
                    treap.insert(x, 1);
                }
            }
            Query1() => {
                let (&min, &c) = treap.min().unwrap();
                if c > 1 {
                    *treap.get_mut(&min).unwrap() -= 1;
                } else {
                    treap.remove(&min);
                }
                println!("{}", min);
            }
            Query2() => {
                let (&max, &c) = treap.max().unwrap();
                if c > 1 {
                    *treap.get_mut(&max).unwrap() -= 1;
                } else {
                    treap.remove(&max);
                }
                println!("{}", max);
            }
        }
    }
}
