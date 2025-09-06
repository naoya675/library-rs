// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

use proconio::input;

use euler_tour::EulerTour;
use fenwick_tree::FenwickTree;

query::define_query! {
    Query {
        0 => Query0(p: usize, x: i64),
        1 => Query1(u: usize, v: usize),
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        uv: [(usize, usize); n - 1],
        queries: [Query; q],
    }
    let mut et = EulerTour::<usize>::new(n);
    uv.iter().for_each(|&(u, v)| {
        et.add_edge(u, v, 0);
        et.add_edge(v, u, 0);
    });
    et.init(0);

    let mut ft = FenwickTree::<i64>::new(n + n);
    for i in 0..n {
        let index = et.index(i);
        ft.add(index.0, a[i]);
        ft.add(index.1, -a[i]);
    }
    for query in queries {
        match query {
            Query0(p, x) => {
                let index = et.index(p);
                ft.add(index.0, x);
                ft.add(index.1, -x);
            }
            Query1(u, v) => {
                let mut res = 0;
                et.for_each(u, v, |l, r| res += ft.sum(l, r));
                println!("{}", res);
            }
        }
    }
}
