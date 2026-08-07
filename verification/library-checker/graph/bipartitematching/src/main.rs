// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bipartitematching

use fast_io::{Output, input, output};

use hopcroft_karp::HopcroftKarp;

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
        l: usize,
        r: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut out = Output::new();

    let mut hk = HopcroftKarp::new(l, r);
    for (a, b) in ab {
        hk.add_edge(a, b);
    }
    let matching = hk.solve();

    output!(out, matching.len());
    for (a, b) in matching {
        output!(out, a, b);
    }
}
