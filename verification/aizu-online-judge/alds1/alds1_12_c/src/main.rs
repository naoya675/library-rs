// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_C

use proconio::input;

use dijkstra::dijkstra;

fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..n {
        input! {
            u: usize,
            k: usize,
            vc: [(usize, i64); k],
        }
        for &(v, c) in &vc {
            graph[u].push((v, c));
        }
    }
    let dist = dijkstra(n, &graph, 0);
    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
