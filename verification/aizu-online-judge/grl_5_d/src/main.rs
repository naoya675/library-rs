// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D

use proconio::input;

use euler_tour::EulerTour;
use fenwick_tree_abstract::FenwickTreeAbstract;

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
    }
    let mut et = EulerTour::<usize>::new(n);
    for i in 0..n {
        input! { k: usize, c: [usize; k], }
        for c in c {
            et.add_edge(i, c, 0);
            et.add_edge(c, i, 0);
        }
    }
    et.init(0);
    let mut ft = FenwickTreeAbstract::<i64>::new(n + n, |a, b| a + b, 0, |a| -a);
    input! { q: usize, }
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { v: usize, w:i64, }
                let index = et.index(v);
                ft.add(index.0, w);
                ft.add(index.1, -w);
            }
            1 => {
                input! { v: usize, }
                let mut res = 0;
                et.path_query_for_edge(0, v, |l, r| res += ft.sum(l, r));
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}
