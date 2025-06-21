// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D

use proconio::input;

use fenwick_tree_abstract::FenwickTreeAbstract;
use heavy_light_decomposition::HeavyLightDecomposition;

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
    let mut hld = HeavyLightDecomposition::<usize>::new(n);
    for i in 0..n {
        input! { k: usize, c: [usize; k], }
        for c in c {
            hld.add_edge(i, c, 0);
            hld.add_edge(c, i, 0);
        }
    }
    hld.init(0);
    let mut ft = FenwickTreeAbstract::<i64>::new(n, |a, b| a + b, 0, |a| -a);
    input! { q: usize, }
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { v: usize, w:i64, }
                let index = hld.index(v);
                ft.add(index.0, w);
            }
            1 => {
                input! { v: usize, }
                let mut res = 0;
                hld.for_each_edge(0, v, |l, r| res += ft.sum(l, r));
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}
