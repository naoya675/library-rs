use std::collections::BinaryHeap;

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
pub struct Dijkstra {
    size: usize,
    graph: Vec<Vec<Edge>>,
}

impl Dijkstra {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.graph[from].push(Edge::new(from, to, cost));
    }

    pub fn dijkstra(&mut self, s: usize) -> Vec<Cost> {
        let mut dist = vec![Cost::MAX / 4; self.size];
        dist[s] = 0;
        let mut heap: BinaryHeap<(Cost, usize)> = BinaryHeap::new();
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

    pub fn dijkstra_prev(&mut self, s: usize) -> (Vec<Cost>, Vec<usize>) {
        let mut dist = vec![Cost::MAX / 4; self.size];
        let mut prev = vec![self.size; self.size];
        dist[s] = 0;
        let mut heap: BinaryHeap<(Cost, usize)> = BinaryHeap::new();
        heap.push((-dist[s], s));
        while let Some((d, from)) = heap.pop() {
            if dist[from] < -d {
                continue;
            }
            for edge in &self.graph[from] {
                if dist[edge.to] > dist[from] + edge.cost {
                    dist[edge.to] = dist[from] + edge.cost;
                    prev[edge.to] = from;
                    heap.push((-dist[edge.to], edge.to));
                }
            }
        }
        (dist, prev)
    }
}

pub fn dijkstra(size: usize, graph: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX / 4; size];
    dist[s] = 0;
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
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
