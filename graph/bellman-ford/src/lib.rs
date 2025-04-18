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
pub struct BellmanFord {
    size: usize,
    edge: Vec<Edge>,
}

impl BellmanFord {
    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn bellman_ford(&mut self, s: usize) -> (bool, Vec<Cost>) {
        let mut dist = vec![Cost::MAX / 4; self.size];
        dist[s] = 0;
        for _ in 0..self.size {
            let mut update = false;
            for edge in &self.edge {
                if dist[edge.from] == Cost::MAX / 4 {
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
                if dist[edge.from] == Cost::MAX / 4 {
                    continue;
                }
                if dist[edge.from] + edge.cost < dist[edge.to] {
                    dist[edge.to] = -(Cost::MAX / 4);
                }
            }
        }
        (true, dist)
    }
}

pub fn bellman_ford(size: usize, edge: &Vec<(usize, usize, i64)>, s: usize) -> (bool, Vec<i64>) {
    let mut dist = vec![i64::MAX / 4; size];
    dist[s] = 0;
    for _ in 0..size {
        let mut update = false;
        for &(from, to, cost) in edge {
            if dist[from] == i64::MAX / 4 {
                continue;
            }
            if dist[from] + cost < dist[to] {
                dist[to] = dist[from] + cost;
                update = true;
            }
        }
        if !update {
            return (false, dist);
        }
    }
    for _ in 0..size {
        for &(from, to, cost) in edge {
            if dist[from] == i64::MAX / 4 {
                continue;
            }
            if dist[from] + cost < dist[to] {
                dist[to] = -(i64::MAX / 4);
            }
        }
    }
    (true, dist)
}
