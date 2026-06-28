// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_path_composite_sum

use proconio::input;

use itertools::Join;
use modint::Modint;
use rerooting::Rerooting;

type Mint = Modint<998244353>;

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
        a: [i64; n],
        edges: [(usize, usize, i64, i64); n - 1],
    }
    let a: Vec<Mint> = a.iter().map(|&a| Mint::new(a)).collect();
    let mut g = Rerooting::<(i64, i64), (Mint, Mint), _, _, _, _>::new(
        n,
        |x, y| (x.0 + y.0, x.1 + y.1),
        || (Mint::new(0), Mint::new(0)),
        |v| (a[v], Mint::new(1)),
        |x, _, _, (b, c)| (Mint::new(b) * x.0 + Mint::new(c) * x.1, x.1),
    );
    for &(u, v, b, c) in &edges {
        g.add_edge(u, v, (b, c));
        g.add_edge(v, u, (b, c));
    }

    let res = g.run();

    println!("{}", res.iter().map(|&d| d.0).join(" "));
}
