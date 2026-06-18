// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bipartitematching

use proconio::input;

use maxflow::Maxflow;

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
        l: usize,
        r: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let s = l + r;
    let t = l + r + 1;
    let mut mf = Maxflow::new(l + r + 2);
    for i in 0..l {
        mf.add_edge(s, i, 1);
    }
    for j in 0..r {
        mf.add_edge(l + j, t, 1);
    }
    let eidx = ab.iter().map(|&(a, b)| mf.add_edge(a, l + b, 1)).collect::<Vec<_>>();
    println!("{}", mf.flow(s, t));
    for i in 0..m {
        if mf.get_edge(eidx[i]).flow > 0 {
            let (a, b) = ab[i];
            println!("{} {}", a, b);
        }
    }
}
