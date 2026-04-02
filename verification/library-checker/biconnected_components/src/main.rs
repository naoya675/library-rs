// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components

use itertools::Itertools;
use proconio::input;

use biconnected_components::BiConnectedComponents;

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
    let mut bcc = BiConnectedComponents::new(n);
    for &(a, b) in &ab {
        bcc.add_edge(a, b);
    }
    bcc.build();

    let mut in_component = vec![false; n];
    let mut components = bcc
        .bc()
        .iter()
        .map(|bc| {
            let mut vec = vec![];
            for &(u, v) in bc {
                vec.push(u);
                vec.push(v);
                in_component[u] = true;
                in_component[v] = true;
            }
            vec.sort_unstable();
            vec.dedup();
            vec
        })
        .collect::<Vec<_>>();
    for v in 0..n {
        if !in_component[v] {
            components.push(vec![v]);
        }
    }

    println!("{}", components.len());
    for comp in &components {
        println!("{} {}", comp.len(), comp.iter().join(" "));
    }
}
