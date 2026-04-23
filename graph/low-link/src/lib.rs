use compressed_sparse_row::Csr;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub id: usize,
}

#[derive(Debug, Clone)]
pub struct LowLink {
    size: usize,
    edges: Vec<(usize, Edge)>,
    graph: Csr<Edge>,
    ord: Vec<usize>,
    low: Vec<usize>,
    used: Vec<bool>,
    articulation: Vec<usize>,    // articulation points
    bridge: Vec<(usize, usize)>, // bridges
}

impl LowLink {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edges: vec![],
            graph: Csr::new(0, &[]),
            ord: vec![],
            low: vec![],
            used: vec![],
            articulation: vec![],
            bridge: vec![],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn graph(&self) -> &Csr<Edge> {
        &self.graph
    }

    pub fn ord(&self, v: usize) -> usize {
        self.ord[v]
    }

    pub fn low(&self, v: usize) -> usize {
        self.low[v]
    }

    pub fn articulation(&self) -> &[usize] {
        &self.articulation
    }

    pub fn bridge(&self) -> &[(usize, usize)] {
        &self.bridge
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.size);
        assert!(v < self.size);
        let id = self.edges.len() / 2;
        self.edges.push((u, Edge { from: u, to: v, id }));
        self.edges.push((v, Edge { from: v, to: u, id }));
    }

    pub fn build(&mut self) {
        self.graph = Csr::new(self.size, &self.edges);
        self.ord = vec![0; self.size];
        self.low = vec![0; self.size];
        self.used = vec![false; self.size];
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
        self.low[v] = k;
        k += 1;
        let mut is_articulation = false;
        let mut count = 0;
        for i in 0..self.graph[v].len() {
            let edge = self.graph[v][i];
            if Some(edge.id) == par {
                continue;
            }
            if !self.used[edge.to] {
                count += 1;
                k = self.dfs(edge.to, k, Some(edge.id));
                self.low[v] = self.low[v].min(self.low[edge.to]);
                if self.ord[v] <= self.low[edge.to] && par.is_some() {
                    is_articulation = true;
                }
                if self.ord[v] < self.low[edge.to] {
                    self.bridge.push((edge.from, edge.to));
                }
            } else {
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
}
