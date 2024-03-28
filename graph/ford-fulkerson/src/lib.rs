#[derive(Debug, Clone, Copy)]
pub struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

impl Edge {
    pub fn new(to: usize, cap: usize, rev: usize) -> Self {
        Self { to, cap, rev }
    }
}

#[derive(Debug, Clone)]
pub struct FordFulkerson {
    size: usize,
    used: Vec<bool>,
    graph: Vec<Vec<Edge>>,
}

impl FordFulkerson {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            used: vec![false; size],
            graph: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let alen = self.graph[a].len();
        let blen = self.graph[b].len();
        self.graph[a].push(Edge::new(b, c, blen));
        self.graph[b].push(Edge::new(a, 0, alen));
    }

    pub fn flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            for i in 0..self.size {
                self.used[i] = false;
            }
            let flow = self.dfs(s, t, usize::MAX);
            if flow == 0 {
                break;
            }
            total_flow += flow;
        }
        total_flow
    }

    fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        if v == t {
            return f;
        }
        self.used[v] = true;
        for i in 0..self.graph[v].len() {
            let u = self.graph[v][i];
            if u.cap == 0 {
                continue;
            }
            if self.used[u.to] {
                continue;
            }
            let flow = self.dfs(u.to, t, f.min(u.cap));
            if flow > 0 {
                self.graph[v][i].cap -= flow;
                self.graph[u.to][u.rev].cap += flow;
                return flow;
            }
        }
        0
    }
}
