// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_C

use proconio::input;

use dijkstra::dijkstra;

fn main() {
    input! {
        n: usize,
        uvc: [(usize, [(usize, i64)]); n],
    }
    let mut graph = vec![vec![]; n];
    for (u, vc) in uvc {
        graph[u] = vc;
    }

    let dist = dijkstra(n, &graph, 0);

    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
