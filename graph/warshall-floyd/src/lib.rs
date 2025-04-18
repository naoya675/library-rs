pub type Cost = i64;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: Cost,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: Cost) -> Self {
        Self { from, to, cost }
    }
}

#[derive(Debug, Clone)]
pub struct WarshallFloyd {
    size: usize,
    edge: Vec<Edge>,
}

impl WarshallFloyd {
    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn warshall_floyd(&mut self) -> (bool, Vec<Vec<Cost>>) {
        let mut dist = vec![vec![Cost::MAX / 4; self.size]; self.size];
        for i in 0..self.size {
            dist[i][i] = 0;
        }
        for edge in &self.edge {
            dist[edge.from][edge.to] = edge.cost;
        }
        for k in 0..self.size {
            for i in 0..self.size {
                for j in 0..self.size {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
                }
            }
        }
        for i in 0..self.size {
            if dist[i][i] < 0 {
                return (true, dist);
            }
        }
        (false, dist)
    }
}

pub fn warshall_floyd(size: usize, edge: &Vec<(usize, usize, i64)>) -> (bool, Vec<Vec<i64>>) {
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
