// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum

use proconio::input;

use implicit_treap::ImplicitTreap;
use modint::Modint;

type Mint = Modint<998244353>;

query::define_query! {
    Query {
        0 => Query0(i: usize, x: i64),
        1 => Query1(i: usize),
        2 => Query2(l: usize, r: usize),
        3 => Query3(l: usize, r: usize, b: i64, c: i64),
        4 => Query4(l: usize, r: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let a = a.iter().map(|&a| (Mint::new(a), Mint::new(1))).collect::<Vec<_>>();
    let mut treap = ImplicitTreap::from_slice(
        &a,
        |x, y| (x.0 + y.0, x.1 + y.1),
        (Mint::new(0), Mint::new(0)),
        |f, x| (f.0 * x.0 + f.1 * x.1, x.1),
        |f, g| (f.0 * g.0, f.0 * g.1 + f.1),
        (Mint::new(1), Mint::new(0)),
    );

    for query in queries {
        match query {
            Query0(i, x) => {
                treap.insert(i, (Mint::new(x), Mint::new(1)));
            }
            Query1(i) => {
                treap.remove(i);
            }
            Query2(l, r) => {
                treap.reverse(l, r);
            }
            Query3(l, r, b, c) => {
                treap.apply(l, r, (Mint::new(b), Mint::new(c)));
            }
            Query4(l, r) => {
                println!("{}", treap.prod(l, r).0);
            }
        }
    }
}
