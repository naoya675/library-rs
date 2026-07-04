use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct HopcroftKarp {
    n: usize,
    m: usize,
    graph: Vec<Vec<usize>>,
    match_l: Vec<Option<usize>>,
    match_r: Vec<Option<usize>>,
    level: Vec<Option<usize>>,
    used: Vec<bool>,
}

impl HopcroftKarp {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            graph: vec![vec![]; n],
            match_l: vec![None; n],
            match_r: vec![None; m],
            level: vec![None; n + 1],
            used: vec![false; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.n);
        assert!(v < self.m);
        self.graph[u].push(v);
    }

    pub fn solve(&mut self) -> Vec<(usize, usize)> {
        loop {
            self.bfs();
            if self.level[self.n].is_none() {
                break;
            }
            self.used.fill(false);
            for u in 0..self.n {
                if self.match_l[u].is_none() {
                    self.dfs(u);
                }
            }
        }
        (0..self.n).filter_map(|u| self.match_l[u].map(|v| (u, v))).collect()
    }

    fn bfs(&mut self) {
        self.level.fill(None);
        let mut que = VecDeque::new();
        for u in 0..self.n {
            if self.match_l[u].is_none() {
                self.level[u] = Some(0);
                que.push_back(u);
            }
        }
        while let Some(u) = que.pop_front() {
            for i in 0..self.graph[u].len() {
                let v = self.graph[u][i];
                let next = self.match_r[v].unwrap_or(self.n);
                if self.level[next].is_none() {
                    self.level[next] = Some(self.level[u].unwrap() + 1);
                    if next != self.n {
                        que.push_back(next);
                    }
                }
            }
        }
    }

    fn dfs(&mut self, u: usize) -> bool {
        if u == self.n {
            return true;
        }
        if self.used[u] {
            return false;
        }
        self.used[u] = true;
        for i in 0..self.graph[u].len() {
            let v = self.graph[u][i];
            let next = self.match_r[v].unwrap_or(self.n);
            if self.level[next] == Some(self.level[u].unwrap() + 1) && self.dfs(next) {
                self.match_l[u] = Some(v);
                self.match_r[v] = Some(u);
                return true;
            }
        }
        false
    }
}
