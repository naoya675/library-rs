pub fn bellman_ford(size: usize, edge: &[(usize, usize, i64)], s: usize) -> (bool, Vec<i64>) {
    let mut dist = vec![i64::MAX; size];
    dist[s] = 0;
    for _ in 0..size {
        let mut update = false;
        for &(from, to, cost) in edge {
            if dist[from] == i64::MAX {
                continue;
            }
            if dist[from].saturating_add(cost) < dist[to] {
                dist[to] = dist[from].saturating_add(cost);
                update = true;
            }
        }
        if !update {
            return (false, dist);
        }
    }
    for _ in 0..size {
        for &(from, to, cost) in edge {
            if dist[from] == i64::MAX {
                continue;
            }
            if dist[from].saturating_add(cost) < dist[to] {
                dist[to] = i64::MIN;
            }
        }
    }
    (true, dist)
}
