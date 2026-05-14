// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use proconio::input;

use dijkstra::dijkstra_with_prev;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        abc: [(usize, usize, i64); m],
    }
    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
    }
    let (dist, prev) = dijkstra_with_prev(n, &graph, s);
    if dist[t] == i64::MAX {
        println!("-1");
        return;
    }
    let mut res = vec![t];
    let mut cur = t;
    while let Some(p) = prev[cur] {
        res.push(p);
        cur = p;
    }
    res.reverse();
    println!("{} {}", dist[t], res.len() - 1);
    for uv in res.windows(2) {
        println!("{} {}", uv[0], uv[1]);
    }
}
