#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: i64) -> Self {
        Self { from, to, cost }
    }
}

#[derive(Debug, Clone)]
pub struct BellmanFord {
    size: usize,
    edge: Vec<Edge>,
}

impl BellmanFord {
    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn bellman_ford(&mut self, s: usize) -> (bool, Vec<i64>) {
        let mut dist = vec![i64::MAX / 4; self.size];
        dist[s] = 0;
        for _ in 0..self.size {
            let mut update = false;
            for edge in &self.edge {
                if dist[edge.from] == i64::MAX / 4 {
                    continue;
                }
                if dist[edge.from] + edge.cost < dist[edge.to] {
                    dist[edge.to] = dist[edge.from] + edge.cost;
                    update = true;
                }
            }
            if !update {
                return (false, dist);
            }
        }
        for _ in 0..self.size {
            for edge in &self.edge {
                if dist[edge.from] == i64::MAX / 4 {
                    continue;
                }
                if dist[edge.from] + edge.cost < dist[edge.to] {
                    dist[edge.to] = -(i64::MAX / 4);
                }
            }
        }
        (true, dist)
    }
}
