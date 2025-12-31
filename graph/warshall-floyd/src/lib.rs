pub fn warshall_floyd(size: usize, edge: &[(usize, usize, i64)]) -> (bool, Vec<Vec<i64>>) {
    let mut dist = vec![vec![i64::MAX / 4; size]; size];
    for i in 0..size {
        dist[i][i] = 0;
    }
    for &(from, to, cost) in edge {
        dist[from][to] = cost;
    }
    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
            }
        }
    }
    let mut cycle = false;
    for i in 0..size {
        if dist[i][i] < 0 {
            cycle = true;
        }
    }
    (cycle, dist)
}
