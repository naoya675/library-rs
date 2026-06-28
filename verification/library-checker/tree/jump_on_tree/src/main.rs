// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use proconio::input;

use heavy_light_decomposition::HeavyLightDecomposition;

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
        q: usize,
        ab: [(usize, usize); n - 1],
        sti: [(usize, usize, usize); q],
    }
    let mut hld = HeavyLightDecomposition::new(n);
    for &(a, b) in &ab {
        hld.add_edge(a, b, 0);
        hld.add_edge(b, a, 0);
    }
    hld.init(0);

    for &(s, t, i) in &sti {
        let d = hld.distance(s, t);
        if i > d {
            println!("-1");
        } else if i <= hld.distance(s, hld.lca(s, t)) {
            println!("{}", hld.la(s, i));
        } else {
            println!("{}", hld.la(t, d - i));
        }
    }
}
