use std::collections::VecDeque;

pub trait CapTrait:
    Copy + PartialOrd + Ord + PartialEq + Eq + std::ops::Add<Output = Self> + std::ops::AddAssign + std::ops::Sub<Output = Self> + std::ops::SubAssign + Default
{
    fn max_value() -> Self;
    fn min_value() -> Self;
}

macro_rules! impl_cap_trait {
    ($($type:ty), *) => {
        $(
            impl CapTrait for $type {
                fn max_value() -> Self { <$type>::MAX }
                fn min_value() -> Self { <$type>::MIN }
            }
        )*
    };
}

impl_cap_trait!(u32, i32, u64, i64, usize, isize);

#[derive(Clone, Debug)]
pub struct Edge<Cap> {
    pub from: usize,
    pub to: usize,
    pub cap: Cap,
    pub flow: Cap,
}

#[derive(Clone, Debug)]
struct InnerEdge<Cap> {
    to: usize,
    rev: usize,
    cap: Cap,
}

pub struct Maxflow<Cap> {
    n: usize,
    g: Vec<Vec<InnerEdge<Cap>>>,
    pos: Vec<(usize, usize)>,
}

impl<Cap: CapTrait> Maxflow<Cap> {
    pub fn new(n: usize) -> Self {
        Maxflow {
            n,
            g: vec![vec![]; n],
            pos: vec![],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        assert!(Cap::default() <= cap);
        let m = self.pos.len();
        self.pos.push((from, self.g[from].len()));
        let from_id = self.g[from].len();
        let to_id = self.g[to].len() + if from == to { 1 } else { 0 };
        self.g[from].push(InnerEdge { to, rev: to_id, cap });
        self.g[to].push(InnerEdge {
            to: from,
            rev: from_id,
            cap: Cap::default(),
        });
        m
    }

    pub fn get_edge(&self, i: usize) -> Edge<Cap> {
        assert!(i < self.pos.len());
        let (from, from_id) = self.pos[i];
        let e = &self.g[from][from_id];
        let re = &self.g[e.to][e.rev];
        Edge {
            from,
            to: e.to,
            cap: e.cap + re.cap,
            flow: re.cap,
        }
    }

    pub fn edges(&self) -> Vec<Edge<Cap>> {
        (0..self.pos.len()).map(|i| self.get_edge(i)).collect()
    }

    pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow: Cap) {
        assert!(i < self.pos.len());
        assert!(Cap::default() <= new_flow);
        assert!(new_flow <= new_cap);
        let (from, from_id) = self.pos[i];
        let e = &self.g[from][from_id];
        let to = e.to;
        let rev = e.rev;
        self.g[from][from_id].cap = new_cap - new_flow;
        self.g[to][rev].cap = new_flow;
    }

    pub fn flow(&mut self, s: usize, t: usize) -> Cap {
        self.flow_with_limit(s, t, Cap::max_value())
    }

    fn bfs(&mut self, s: usize, t: usize, que: &mut VecDeque<usize>, level: &mut [i64]) {
        level.fill(-1);
        level[s] = 0;
        que.clear();
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in &self.g[v] {
                if e.cap == Cap::default() || level[e.to] >= 0 {
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

    fn dfs(&mut self, s: usize, t: usize, v: usize, up: Cap, level: &mut [i64], iter: &mut [usize]) -> Cap {
        if v == s {
            return up;
        }
        let mut res = Cap::default();
        let level_v = level[v];
        while iter[v] < self.g[v].len() {
            let i = iter[v];
            let e = &self.g[v][i];
            let to = e.to;
            let rev = e.rev;
            if level_v <= level[to] || self.g[to][rev].cap == Cap::default() {
                iter[v] += 1;
                continue;
            }
            let cap = self.g[to][rev].cap;
            let d = self.dfs(s, t, to, std::cmp::min(up - res, cap), level, iter);
            if d <= Cap::default() {
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

    pub fn flow_with_limit(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
        assert!(s < self.n);
        assert!(t < self.n);
        assert!(s != t);

        let mut level = vec![0; self.n];
        let mut iter = vec![0; self.n];
        let mut que = VecDeque::new();

        let mut flow = Cap::default();
        while flow < flow_limit {
            self.bfs(s, t, &mut que, &mut level);
            if level[t] == -1 {
                break;
            }

            iter.fill(0);
            let f = self.dfs(s, t, t, flow_limit - flow, &mut level, &mut iter);
            if f == Cap::default() {
                break;
            }
            flow += f;
        }
        flow
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self.n];
        let mut que = VecDeque::new();
        que.push_back(s);
        while let Some(p) = que.pop_front() {
            visited[p] = true;
            for e in &self.g[p] {
                if e.cap > Cap::default() && !visited[e.to] {
                    visited[e.to] = true;
                    que.push_back(e.to);
                }
            }
        }
        visited
    }
}
