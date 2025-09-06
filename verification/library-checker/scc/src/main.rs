// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc

use itertools::Itertools;
use proconio::input;

use strongly_connected_components::StronglyConnectedComponents;

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
    let mut scc = StronglyConnectedComponents::new(n);
    ab.iter().for_each(|&(a, b)| scc.add_edge(a, b));
    let groups = scc.scc();

    println!("{}", groups.len());
    for group in groups {
        println!("{} {}", group.len(), group.iter().join(" "));
    }
}
