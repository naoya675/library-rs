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
    ord: Vec<usize>,
    low: Vec<usize>,
    used: Vec<bool>,
    articulation: Vec<usize>, // articulation points
    bridge: Vec<Edge>,        // bridges
}

impl LowLink {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
            ord: vec![0; size],
            low: vec![0; size],
            used: vec![false; size],
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
            if !self.used[i] {
                k = self.dfs(i, k, None);
            }
        }
    }

    fn dfs(&mut self, v: usize, mut k: usize, par: Option<usize>) -> usize {
        self.used[v] = true;
        self.ord[v] = k;
        self.low[v] = self.ord[v];
        k += 1;
        let mut is_articulation = false;
        let mut count = 0; // number of child
        for &edge in &self.graph[v].clone() {
            if !self.used[edge.to] {
                count += 1;
                k = self.dfs(edge.to, k, Some(v));
                self.low[v] = self.low[v].min(self.low[edge.to]);
                if par.is_some() && self.ord[v] <= self.low[edge.to] {
                    is_articulation = true;
                }
                if self.ord[v] < self.low[edge.to] {
                    self.bridge.push(edge);
                }
            } else if Some(edge.to) != par {
                self.low[v] = self.low[v].min(self.ord[edge.to]);
            }
        }
        if par.is_none() && count > 1 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulation.push(v);
        }
        k
    }

    pub fn articulation(&self) -> Vec<usize> {
        self.articulation.clone()
    }

    pub fn bridge(&self) -> Vec<(usize, usize)> {
        self.bridge.iter().map(|&edge| (edge.from, edge.to)).collect()
    }
}
