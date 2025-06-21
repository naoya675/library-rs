// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C

use proconio::input;

use heavy_light_decomposition::HeavyLightDecomposition;

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
        for c in c {
            hld.add_edge(i, c, 0);
            hld.add_edge(c, i, 0);
        }
    }
    hld.init(0);
    input! { q: usize, }
    for _ in 0..q {
        input! { u: usize, v: usize, }
        println!("{}", hld.lca(u, v));
    }
}
