// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_B

use proconio::input;

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
    input! {
        n: usize,
        stw: [(usize, usize, usize); n - 1],
    }
    let mut g = Rerooting::<usize, usize, _, _, _, _>::new(
        n,
        |a: usize, b: usize| std::cmp::max(a, b),
        || 0,
        || 0,
        |a: usize, _: usize, _: usize, w: usize| a + w,
    );
    stw.iter().for_each(|&(s, t, w)| {
        g.add_edge(s, t, w);
        g.add_edge(t, s, w);
    });

    for res in g.run() {
        println!("{}", res);
    }
}
