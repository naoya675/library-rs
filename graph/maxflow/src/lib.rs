use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cap: i64,
    pub flow: i64,
}

#[derive(Clone, Debug)]
struct InnerEdge {
    to: usize,
    rev: usize,
    cap: i64,
}

pub struct Maxflow {
    n: usize,
    g: Vec<Vec<InnerEdge>>,
    pos: Vec<(usize, usize)>,
}

impl Maxflow {
    pub fn new(n: usize) -> Self {
        Maxflow {
            n,
            g: vec![vec![]; n],
            pos: vec![],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        assert!(0 <= cap);
        let m = self.pos.len();
        self.pos.push((from, self.g[from].len()));
        let from_id = self.g[from].len();
        let to_id = self.g[to].len() + if from == to { 1 } else { 0 };
        self.g[from].push(InnerEdge { to: to, rev: to_id, cap: cap });
        self.g[to].push(InnerEdge {
            to: from,
            rev: from_id,
            cap: 0,
        });
        m
    }

    pub fn get_edge(&self, i: usize) -> Edge {
        assert!(i < self.pos.len());
        let (from, from_id) = self.pos[i];
        let e = &self.g[from][from_id];
        let re = &self.g[e.to][e.rev];
        Edge {
            from: from,
            to: e.to,
            cap: e.cap + re.cap,
            flow: re.cap,
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        (0..self.pos.len()).map(|i| self.get_edge(i)).collect()
    }

    pub fn change_edge(&mut self, i: usize, new_cap: i64, new_flow: i64) {
        assert!(i < self.pos.len());
        assert!(0 <= new_flow);
        assert!(new_flow <= new_cap);
        let (from, from_id) = self.pos[i];
        let e = &self.g[from][from_id];
        let to = e.to;
        let rev = e.rev;
        self.g[from][from_id].cap = new_cap - new_flow;
        self.g[to][rev].cap = new_flow;
    }

    pub fn flow(&mut self, s: usize, t: usize) -> i64 {
        self.flow_with(s, t, i64::MAX)
    }

    pub fn flow_with(&mut self, s: usize, t: usize, flow_limit: i64) -> i64 {
        assert!(s < self.n);
        assert!(t < self.n);
        assert!(s != t);

        let mut level = vec![0; self.n];
        let mut iter = vec![0; self.n];
        let mut que = VecDeque::new();

        let mut flow = 0;
        while flow < flow_limit {
            self.bfs(s, t, &mut que, &mut level);
            if level[t] == -1 {
                break;
            }

            iter.fill(0);
            let f = self.dfs(s, t, t, flow_limit - flow, &mut level, &mut iter);
            if f == 0 {
                break;
            }
            flow += f;
        }
        flow
    }

    fn bfs(&mut self, s: usize, t: usize, que: &mut VecDeque<usize>, level: &mut [i64]) {
        level.fill(-1);
        level[s] = 0;
        que.clear();
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in &self.g[v] {
                if e.cap == 0 || level[e.to] >= 0 {
                    continue;
                }
                level[e.to] = level[v] + 1;
                if e.to == t {
                    return;
                }
                que.push_back(e.to);
            }
        }
    }

    fn dfs(&mut self, s: usize, t: usize, v: usize, up: i64, level: &mut [i64], iter: &mut [usize]) -> i64 {
        if v == s {
            return up;
        }
        let mut res = 0;
        let level_v = level[v];
        while iter[v] < self.g[v].len() {
            let i = iter[v];
            let e = &self.g[v][i];
            let to = e.to;
            let rev = e.rev;
            if level_v <= level[to] || self.g[to][rev].cap == 0 {
                iter[v] += 1;
                continue;
            }
            let cap = self.g[to][rev].cap;
            let d = self.dfs(s, t, to, std::cmp::min(up - res, cap), level, iter);
            if d <= 0 {
                iter[v] += 1;
                continue;
            }
            self.g[v][i].cap += d;
            self.g[to][rev].cap -= d;
            res += d;
            if res == up {
                return res;
            }
            iter[v] += 1;
        }
        level[v] = self.n as i64;
        res
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self.n];
        let mut que = VecDeque::new();
        que.push_back(s);
        while let Some(p) = que.pop_front() {
            visited[p] = true;
            for e in &self.g[p] {
                if e.cap > 0 && !visited[e.to] {
                    visited[e.to] = true;
                    que.push_back(e.to);
                }
            }
        }
        visited
    }
}
