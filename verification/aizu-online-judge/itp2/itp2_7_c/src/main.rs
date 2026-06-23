// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_7_C

use proconio::input;

use treap::Treap;

query::define_query! {
    Query {
        0 => Query0(x: usize),
        1 => Query1(x: usize),
        2 => Query2(x: usize),
        3 => Query3(l: usize, r: usize), // dump [l, r]
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut treap = Treap::new();
    for query in queries {
        match query {
            Query0(x) => {
                treap.insert(x);
                println!("{}", treap.len());
            }
            Query1(x) => {
                println!("{}", if treap.contains(&x) { 1 } else { 0 });
            }
            Query2(x) => {
                treap.remove(&x);
            }
            Query3(l, r) => {
                let lo = treap.lower_bound(&l);
                let hi = treap.upper_bound(&r);
                for k in lo..hi {
                    println!("{}", treap.kth(k));
                }
            }
        }
    }
}
