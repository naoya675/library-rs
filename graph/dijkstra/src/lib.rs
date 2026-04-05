use std::collections::BinaryHeap;

pub fn dijkstra(size: usize, graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; size];
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    dist[s] = 0;
    heap.push((-dist[s], s));
    while let Some((d, from)) = heap.pop() {
        if dist[from] < -d {
            continue;
        }
        for &(to, cost) in &graph[from] {
            if dist[to] > dist[from].saturating_add(cost) {
                dist[to] = dist[from].saturating_add(cost);
                heap.push((-dist[to], to));
            }
        }
    }
    dist
}

pub fn dijkstra_with_prev(size: usize, graph: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    let mut dist = vec![i64::MAX; size];
    let mut prev = vec![None; size];
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    dist[s] = 0;
    heap.push((-dist[s], s));
    while let Some((d, from)) = heap.pop() {
        if dist[from] < -d {
            continue;
        }
        for &(to, cost) in &graph[from] {
            if dist[to] > dist[from].saturating_add(cost) {
                dist[to] = dist[from].saturating_add(cost);
                prev[to] = Some(from);
                heap.push((-dist[to], to));
            }
        }
    }
    (dist, prev)
}
