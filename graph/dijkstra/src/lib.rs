use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(size: usize, graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; size];
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    dist[s] = 0;
    heap.push(Reverse((dist[s], s)));
    while let Some(Reverse((d, from))) = heap.pop() {
        if dist[from] < d {
            continue;
        }
        for &(to, cost) in &graph[from] {
            if dist[to] > dist[from].saturating_add(cost) {
                dist[to] = dist[from].saturating_add(cost);
                heap.push(Reverse((dist[to], to)));
            }
        }
    }
    dist
}

pub fn dijkstra_with_prev(size: usize, graph: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    let mut dist = vec![i64::MAX; size];
    let mut prev = vec![None; size];
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    dist[s] = 0;
    heap.push(Reverse((dist[s], s)));
    while let Some(Reverse((d, from))) = heap.pop() {
        if dist[from] < d {
            continue;
        }
        for &(to, cost) in &graph[from] {
            if dist[to] > dist[from].saturating_add(cost) {
                dist[to] = dist[from].saturating_add(cost);
                prev[to] = Some(from);
                heap.push(Reverse((dist[to], to)));
            }
        }
    }
    (dist, prev)
}
