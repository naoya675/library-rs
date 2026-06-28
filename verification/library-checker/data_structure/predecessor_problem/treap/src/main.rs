// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::{input, marker::Chars};

use treap::Treap;

query::define_query! {
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
        _n: usize,
        q: usize,
        t: Chars,
        queries: [Query; q],
    }
    let mut treap = Treap::new();
    for (i, &c) in t.iter().enumerate() {
        if c == '1' {
            treap.insert(i);
        }
    }

    for query in queries {
        match query {
            Query0(k) => {
                if !treap.contains(&k) {
                    treap.insert(k);
                }
            }
            Query1(k) => {
                treap.remove(&k);
            }
            Query2(k) => {
                println!("{}", if treap.contains(&k) { 1 } else { 0 });
            }
            Query3(k) => {
                println!("{}", treap.successor(&k).map(|&x| x as i64).unwrap_or(-1));
            }
            Query4(k) => {
                println!("{}", treap.predecessor(&k).map(|&x| x as i64).unwrap_or(-1));
            }
        }
    }
}
