// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C

use proconio::input;

use euler_tour::EulerTour;

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
    input! { q: usize, }
    for _ in 0..q {
        input! { u: usize, v: usize, }
        println!("{}", et.lca(u, v));
    }
}
