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
    pub const INF: Cost = Cost::MAX / 2;

    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn warshall_floyd(&mut self) -> (bool, Vec<Vec<Cost>>) {
        let mut dist = vec![vec![Self::INF; self.size]; self.size];
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
