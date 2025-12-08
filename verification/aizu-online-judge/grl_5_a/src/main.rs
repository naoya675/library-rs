// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A

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
        |x: usize, y: usize| std::cmp::max(x, y),
        || 0,
        |_: usize| 0,
        |x: usize, _: usize, _: usize, w: usize| x + w,
    );
    stw.iter().for_each(|&(s, t, w)| {
        g.add_edge(s, t, w);
        g.add_edge(t, s, w);
    });

    let res = g.run();
    println!("{}", res.iter().max().unwrap());
}
