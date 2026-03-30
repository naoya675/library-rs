// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1595

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
        uv: [(usize, usize); n - 1],
    }
    let mut g = Rerooting::<usize, usize, _, _, _, _>::new(
        n,
        |x: usize, y: usize| std::cmp::max(x, y),
        || 0,
        |_: usize| 0,
        |x: usize, _: usize, _: usize, w: usize| x + w,
    );
    uv.iter().for_each(|&(u, v)| {
        g.add_edge(u - 1, v - 1, 1);
        g.add_edge(v - 1, u - 1, 1);
    });

    let res = g.run();
    for i in 0..n {
        println!("{}", (n - 1) * 2 - res[i]);
    }
}
