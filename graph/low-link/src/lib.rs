#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone)]
pub struct LowLink {
    size: usize,
    graph: Vec<Vec<Edge>>,
    visited: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    articulation: Vec<usize>, // articulation point
    bridge: Vec<Edge>,
}

impl LowLink {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
            visited: vec![false; size],
            ord: vec![0; size],
            low: vec![0; size],
            articulation: vec![],
            bridge: vec![],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.size);
        assert!(to < self.size);
        self.graph[from].push(Edge::new(from, to));
    }

    pub fn build(&mut self) {
        let mut k = 0;
        for i in 0..self.size {
            if !self.visited[i] {
                k = self.dfs(i, k, None);
            }
        }
    }

    fn dfs(&mut self, i: usize, mut k: usize, par: Option<usize>) -> usize {
        self.visited[i] = true;
        self.ord[i] = k;
        self.low[i] = self.ord[i];
        k += 1;
        let mut is_articulation = false;
        let mut count = 0; // child
        for &edge in &self.graph[i].clone() {
            if !self.visited[edge.to] {
                count += 1;
                k = self.dfs(edge.to, k, Some(i));
                self.low[i] = self.low[i].min(self.low[edge.to]);
                if par.is_some() && self.ord[i] <= self.low[edge.to] {
                    is_articulation = true;
                }
                if self.ord[i] < self.low[edge.to] {
                    self.bridge.push(edge);
                }
            } else if Some(edge.to) != par {
                self.low[i] = self.low[i].min(self.ord[edge.to]);
            }
        }
        if par.is_none() && count >= 2 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulation.push(i);
        }
        k
    }
}
