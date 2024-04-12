use std::collections::BinaryHeap;

pub type Cost = i64;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    to: usize,
    cost: Cost,
}

impl Edge {
    pub fn new(to: usize, cost: Cost) -> Self {
        Self { to, cost }
    }
}

#[derive(Debug, Clone)]
pub struct Dijkstra {
    size: usize,
    graph: Vec<Vec<Edge>>,
}

impl Dijkstra {
    pub const INF: Cost = Cost::MAX / 2;

    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.graph[from].push(Edge::new(to, cost));
    }

    pub fn dijkstra(&mut self, s: usize) -> Vec<Cost> {
        let mut dist = vec![Self::INF; self.size];
        dist[s] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((-dist[s], s));
        while let Some((d, from)) = heap.pop() {
            if dist[from] < -d {
                continue;
            }
            for edge in &self.graph[from] {
                if dist[edge.to] > dist[from] + edge.cost {
                    dist[edge.to] = dist[from] + edge.cost;
                    heap.push((-dist[edge.to], edge.to));
                }
            }
        }
        dist
    }
}
