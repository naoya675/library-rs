use std::collections::BinaryHeap;

pub fn dijkstra(size: usize, graph: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX / 4; size];
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    dist[s] = 0;
    heap.push((-dist[s], s));
    while let Some((d, from)) = heap.pop() {
        if dist[from] < -d {
            continue;
        }
        for &(to, cost) in &graph[from] {
            if dist[to] > dist[from] + cost {
                dist[to] = dist[from] + cost;
                heap.push((-dist[to], to));
            }
        }
    }
    dist
}
