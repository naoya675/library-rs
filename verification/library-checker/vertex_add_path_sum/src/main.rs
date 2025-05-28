// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

use proconio::input;

use euler_tour::EulerTour;
use fenwick_tree::FenwickTree;

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
    }
    let mut et = EulerTour::<usize>::new(n);
    for (u, v) in uv {
        et.add_edge(u, v, 0);
        et.add_edge(v, u, 0);
    }
    et.init(0);
    let mut ft = FenwickTree::<i64>::new(n + n);
    for i in 0..n {
        let index = et.index(i);
        ft.add(index.0, a[i]);
        ft.add(index.1, -a[i]);
    }
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { p: usize, x: i64, }
                let index = et.index(p);
                ft.add(index.0, x);
                ft.add(index.1, -x);
            }
            1 => {
                input! { u: usize, v: usize, }
                let mut res = 0;
                et.path_vertex(u, v, |l, r| res += ft.sum(l, r));
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}
