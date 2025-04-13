// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A

use rerooting::Rerooting;

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    proconio::input! {
        n: usize,
        stw: [(usize, usize, usize); n - 1],
    }
    let merge = |a: usize, b: usize| std::cmp::max(a, b);
    let e = || 0_usize;
    let leaf = || 0_usize;
    let apply = |a: usize, _: usize, _: usize, w: usize| -> usize { a + w };
    let mut g = Rerooting::<usize, usize, _, _, _, _>::new(n, merge, e, leaf, apply);
    for (s, t, w) in stw {
        g.add_edge(s, t, w);
        g.add_edge(t, s, w);
    }
    let res = g.run();
    println!("{}", res.iter().max().unwrap());
}
