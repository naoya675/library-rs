// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E

use fast_io::{Output, define_query, input, output};

use fenwick_tree_range_add::FenwickTreeRangeAdd;
use heavy_light_decomposition::HeavyLightDecomposition;

define_query! {
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
        c: [[usize]; n],
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut hld = HeavyLightDecomposition::new(n);
    for (i, c) in c.iter().enumerate() {
        for &c in c {
            hld.add_edge(i, c, 0);
            hld.add_edge(c, i, 0);
        }
    }
    hld.init(0);
    let mut ft = FenwickTreeRangeAdd::new(n);

    for query in queries {
        match query {
            Query0(v, w) => {
                hld.for_each_edge(0, v, |l, r| ft.add(l, r, w));
            }
            Query1(v) => {
                let mut res = 0;
                hld.for_each_edge(0, v, |l, r| res += ft.sum(l, r));
                output!(out, res);
            }
        }
    }
}
