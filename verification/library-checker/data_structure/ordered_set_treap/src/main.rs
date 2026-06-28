// verification-helper: PROBLEM https://judge.yosupo.jp/problem/ordered_set

use proconio::input;

use treap::Treap;

query::define_query! {
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
    let mut treap = Treap::new();
    for &x in &a {
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
                    println!("{}", treap.kth(x - 1));
                } else {
                    println!("-1");
                }
            }
            Query3(x) => {
                println!("{}", treap.upper_bound(&x));
            }
            Query4(x) => {
                println!("{}", treap.predecessor(&x).map(|&y| y as i64).unwrap_or(-1));
            }
            Query5(x) => {
                println!("{}", treap.successor(&x).map(|&y| y as i64).unwrap_or(-1));
            }
        }
    }
}
