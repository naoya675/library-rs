// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E

use proconio::input;

use heavy_light_decomposition::HeavyLightDecomposition;
use lazy_segment_tree::LazySegmentTree;

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
    let mut hld = HeavyLightDecomposition::new(n);
    for i in 0..n {
        input! { k: usize, c: [usize; k], }
        for c in c {
            hld.add_edge(i, c, 0);
            hld.add_edge(c, i, 0);
        }
    }
    hld.init(0);
    let mut lst = LazySegmentTree::<(i64, i64), i64>::new(n + n, |a, b| (a.0 + b.0, a.1 + b.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, x| f + x, 0);
    lst.build(vec![(0, 1); n + n]);
    input! { q: usize, }
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { v: usize, w: i64, }
                hld.for_each_edge(0, v, |l, r| {
                    lst.apply(l, r, w);
                });
            }
            1 => {
                input! { v: usize, }
                let mut res = 0;
                hld.for_each_edge(0, v, |l, r| {
                    res += lst.prod(l, r).0;
                });
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}
