// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use fast_io::{Output, input, output};

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
        q: usize,
        p: [usize; n - 1],
        uv: [(usize, usize); q],
    }
    let mut out = Output::new();

    let mut et = EulerTour::new(n);
    for (i, &p) in p.iter().enumerate() {
        et.add_edge(i + 1, p, 0);
        et.add_edge(p, i + 1, 0);
    }
    et.init(0);

    for (u, v) in uv {
        output!(out, et.lca(u, v));
    }
}
