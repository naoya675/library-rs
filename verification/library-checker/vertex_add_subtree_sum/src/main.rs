// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

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
        p: [usize; n - 1],
    }
    let mut et = EulerTour::<usize>::new(n);
    for (i, &p) in p.iter().enumerate() {
        et.add_edge(i + 1, p, 0);
        et.add_edge(p, i + 1, 0);
    }
    et.init(0);
    let mut ft = FenwickTree::new(n + n);
    for i in 0..n {
        let index = et.index(i);
        ft.add(index.0, a[i]);
    }
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { p: usize, x: i64, }
                let index = et.index(p);
                ft.add(index.0, x);
            }
            1 => {
                input! { u: usize, }
                let mut res = 0;
                et.subtree_vertex(u, |l, r| res += ft.sum(l, r));
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}
