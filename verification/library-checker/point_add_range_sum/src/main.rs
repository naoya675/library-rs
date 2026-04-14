// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;

use fenwick_tree::FenwickTree;

query::define_query! {
    Query {
        0 => Query0(p: usize, x: i64),
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
    let mut ft = FenwickTree::new(n);
    for i in 0..n {
        ft.add(i, a[i]);
    }

    for query in queries {
        match query {
            Query0(p, x) => {
                ft.add(p, x);
            }
            Query1(l, r) => {
                println!("{}", ft.sum(l, r));
            }
        }
    }
}
