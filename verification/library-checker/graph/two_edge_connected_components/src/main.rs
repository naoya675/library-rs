// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components

use fast_io::{Output, input, output};

use two_edge_connected_components::TwoEdgeConnectedComponents;

fn main() {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut out = Output::new();

    let mut ecc = TwoEdgeConnectedComponents::new(n);
    for (a, b) in ab {
        ecc.add_edge(a, b);
    }

    ecc.build();
    let group = ecc.group();

    output!(out, group.len());
    for g in group {
        out.print(g.len());
        out.print(" ");
        out.println_iter(g, " ");
    }
}
