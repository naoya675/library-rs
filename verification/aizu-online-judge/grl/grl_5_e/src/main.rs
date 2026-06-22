// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E

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
    let mut hld = HeavyLightDecomposition::new(n);
    for i in 0..n {
        input! { k: usize, c: [usize; k], }
        for c in c {
            hld.add_edge(i, c, 0);
            hld.add_edge(c, i, 0);
        }
    }
    hld.init(0);

    let mut ft = vec![FenwickTreeAbstract::<i64>::new(n + 1, |x, y| x + y, 0, |x| -x); 2];
    input! {
        q: usize,
        queries: [Query; q],
    }
    for query in queries {
        match query {
            Query0(v, w) => {
                hld.for_each_edge(0, v, |l, r| {
                    ft[0].add(l, -w * l as i64);
                    ft[0].add(r, w * r as i64);
                    ft[1].add(l, w);
                    ft[1].add(r, -w);
                });
            }
            Query1(v) => {
                let mut res = 0;
                hld.for_each_edge(0, v, |l, r| {
                    let sum_l = ft[0].sum(0, l) + ft[1].sum(0, l) * l as i64;
                    let sum_r = ft[0].sum(0, r) + ft[1].sum(0, r) * r as i64;
                    res += sum_r - sum_l;
                });
                println!("{}", res);
            }
        }
    }
}
