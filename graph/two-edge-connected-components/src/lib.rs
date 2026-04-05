use low_link::LowLink;

#[derive(Debug, Clone)]
pub struct TwoEdgeConnectedComponents {
    low_link: LowLink,
    used: Vec<bool>,
    comp: Vec<usize>,
    tree: Vec<Vec<usize>>,
    group: Vec<Vec<usize>>,
}

impl TwoEdgeConnectedComponents {
    pub fn new(size: usize) -> Self {
        Self {
            low_link: LowLink::new(size),
            used: vec![],
            comp: vec![],
            tree: vec![],
            group: vec![],
        }
    }

    pub fn comp(&self, v: usize) -> usize {
        self.comp[v]
    }

    pub fn tree(&self) -> &[Vec<usize>] {
        &self.tree
    }

    pub fn group(&self) -> &[Vec<usize>] {
        &self.group
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.low_link.add_edge(u, v);
    }

    pub fn build(&mut self) {
        self.low_link.build();
        self.used = vec![false; self.low_link.size()];
        self.comp = vec![0; self.low_link.size()];
        let mut k = 0;
        for i in 0..self.low_link.size() {
            if !self.used[i] {
                k = self.dfs(i, k, None);
            }
        }
        self.group.resize(k, vec![]);
        for i in 0..self.low_link.size() {
            self.group[self.comp[i]].push(i);
        }
        self.tree.resize(k, vec![]);
        for &(u, v) in self.low_link.bridge() {
            self.tree[self.comp[u]].push(self.comp[v]);
            self.tree[self.comp[v]].push(self.comp[u]);
        }
    }

    fn dfs(&mut self, v: usize, mut k: usize, par: Option<usize>) -> usize {
        self.used[v] = true;
        if let Some(par) = par {
            if self.low_link.ord(par) >= self.low_link.low(v) {
                self.comp[v] = self.comp[par];
            } else {
                self.comp[v] = k;
                k += 1;
            }
        } else {
            self.comp[v] = k;
            k += 1;
        }
        for i in 0..self.low_link.graph()[v].len() {
            let edge = self.low_link.graph()[v][i];
            if !self.used[edge.to] {
                k = self.dfs(edge.to, k, Some(v));
            }
        }
        k
    }
}
