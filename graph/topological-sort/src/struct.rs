use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone)]
pub struct TopologicalSort {
    size: usize,
    graph: Vec<Vec<usize>>,
}

impl TopologicalSort {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
    }

    pub fn topological_sort(&mut self) -> Vec<usize> {
        let mut indegree = vec![0; self.size];
        for from in 0..self.size {
            for i in 0..self.graph[from].len() {
                let to = self.graph[from][i];
                indegree[to] += 1;
            }
        }
        let mut heap = BinaryHeap::new();
        for from in 0..self.size {
            if indegree[from] == 0 {
                heap.push(Reverse(from));
            }
        }
        let mut res = vec![];
        while let Some(Reverse(from)) = heap.pop() {
            res.push(from);
            for i in 0..self.graph[from].len() {
                let to = self.graph[from][i];
                indegree[to] -= 1;
                if indegree[to] == 0 {
                    heap.push(Reverse(to));
                }
            }
        }
        if res.len() != self.size {
            return vec![];
        }
        res
    }
}
