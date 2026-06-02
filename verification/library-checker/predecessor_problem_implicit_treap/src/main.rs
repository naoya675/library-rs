// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::{input, marker::Chars};

use implicit_treap::ImplicitTreap;

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
        n: usize,
        q: usize,
        t: Chars,
        queries: [Query; q],
    }
    let t = t.iter().map(|&t| t as usize - '0' as usize).collect::<Vec<_>>();
    let mut treap = ImplicitTreap::from_slice(&t, |x, y| x + y, 0, |f, x| f + x, |f, g| f + g, 0);
    for query in queries {
        match query {
            Query0(k) => {
                treap.set(k, 1);
            }
            Query1(k) => {
                treap.set(k, 0);
            }
            Query2(k) => {
                println!("{}", treap.get(k));
            }
            Query3(k) => {
                let r = treap.max_right(k, |x| x == 0);
                if r < n {
                    println!("{}", r);
                } else {
                    println!("-1");
                }
            }
            Query4(k) => {
                let l = treap.min_left(k + 1, |x| x == 0);
                if l > 0 {
                    println!("{}", l - 1);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
