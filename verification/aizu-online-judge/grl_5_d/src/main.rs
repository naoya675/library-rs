// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D

use proconio::input;

use fenwick_tree_abstract::FenwickTreeAbstract;
use heavy_light_decomposition::HeavyLightDecomposition;

query::define_query! {
    Query {
        0 => Query0(v: usize, w: i64),
        1 => Query1(v: usize),
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
    }
    let mut hld = HeavyLightDecomposition::<usize>::new(n);
    for i in 0..n {
        input! { k: usize, c: [usize; k], }
        c.iter().for_each(|&c| {
            hld.add_edge(i, c, 0);
            hld.add_edge(c, i, 0);
        });
    }
    hld.init(0);

    let mut ft = FenwickTreeAbstract::<i64>::new(n, |a, b| a + b, 0, |a| -a);
    input! {
        q: usize,
        queries: [Query; q],
    }
    for query in queries {
        match query {
            Query0(v, w) => ft.add(hld.index(v).0, w),
            Query1(v) => {
                let mut res = 0;
                hld.for_each_edge(0, v, |l, r| res += ft.sum(l, r));
                println!("{}", res);
            }
        }
    }
}
