// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_diameter

use proconio::input;

use itertools::Join;
use rerooting::Rerooting;

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
        edges: [(usize, usize, i64); n - 1],
    }
    let mut tree = vec![vec![]; n];
    let mut g = Rerooting::<i64, i64, _, _, _, _>::new(n, |x, y| std::cmp::max(x, y), || 0, |_| 0, |x, _, _, w| x + w);
    for &(a, b, c) in &edges {
        tree[a].push((b, c));
        tree[b].push((a, c));
        g.add_edge(a, b, c);
        g.add_edge(b, a, c);
    }

    let ecc = g.run();
    let diameter = *ecc.iter().max().unwrap();
    let u = ecc.iter().position(|&d| d == diameter).unwrap();
    let mut dist = vec![-1; n];
    let mut par = vec![None; n];
    dist[u] = 0;
    let mut stack = vec![u];
    while let Some(from) = stack.pop() {
        for &(to, c) in &tree[from] {
            if dist[to] == -1 {
                dist[to] = dist[from] + c;
                par[to] = Some(from);
                stack.push(to);
            }
        }
    }
    let v = (0..n).max_by_key(|&i| dist[i]).unwrap();
    let mut res = vec![v];
    let mut cur = v;
    while let Some(p) = par[cur] {
        res.push(p);
        cur = p;
    }
    res.reverse();

    println!("{} {}", diameter, res.len());
    println!("{}", res.iter().join(" "));
}
