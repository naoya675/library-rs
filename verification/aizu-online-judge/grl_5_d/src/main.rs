// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D

use proconio::input;

use euler_tour::EulerTour;
use segment_tree::SegmentTree;

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
    let mut st = SegmentTree::<i64>::new(n + n, |a, b| a + b, 0);
    input! { q: usize, }
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { a: usize, b:i64, }
                let index = et.index(a);
                st.apply(index.0, b);
                st.apply(index.1, -b);
            }
            1 => {
                input! { a: usize, }
                let mut res = 0;
                et.path_edge(0, a, |l, r| res += st.prod(l, r));
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}
