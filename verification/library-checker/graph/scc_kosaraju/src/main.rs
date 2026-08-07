// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc

use fast_io::{Output, input, output};

use strongly_connected_components_kosaraju::StronglyConnectedComponents;

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
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut out = Output::new();

    let mut scc = StronglyConnectedComponents::new(n);
    for (a, b) in ab {
        scc.add_edge(a, b);
    }
    let groups = scc.scc();

    output!(out, groups.len());
    for group in groups {
        out.print(group.len());
        out.print(" ");
        out.println_iter(&group, " ");
    }
}
