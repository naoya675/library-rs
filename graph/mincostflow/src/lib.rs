use std::cmp::Reverse;
use std::collections::BinaryHeap;

use compressed_sparse_row::Csr;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cap: i64,
    pub flow: i64,
    pub cost: i64,
}

#[derive(Clone, Copy, Default)]
struct InnerEdge {
    to: usize,
    rev: usize,
    cap: i64,
    cost: i64,
}

pub struct MinCostFlow {
    n: usize,
    edges: Vec<Edge>,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        Self { n, edges: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64, cost: i64) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        assert!(0 <= cap);
        assert!(0 <= cost);
        let m = self.edges.len();
        self.edges.push(Edge {
            from: from,
            to: to,
            cap: cap,
            flow: 0,
            cost: cost,
        });
        m
    }

    pub fn get_edge(&self, i: usize) -> Edge {
        assert!(i < self.edges.len());
        self.edges[i]
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn flow(&mut self, s: usize, t: usize) -> (i64, i64) {
        self.flow_with(s, t, i64::MAX)
    }

    pub fn flow_with(&mut self, s: usize, t: usize, flow_limit: i64) -> (i64, i64) {
        self.slope_with(s, t, flow_limit).pop().unwrap()
    }

    pub fn slope(&mut self, s: usize, t: usize) -> Vec<(i64, i64)> {
        self.slope_with(s, t, i64::MAX)
    }

    pub fn slope_with(&mut self, s: usize, t: usize, flow_limit: i64) -> Vec<(i64, i64)> {
        assert!(s < self.n);
        assert!(t < self.n);
        assert!(s != t);

        let m = self.edges.len();
        let mut edge_idx = vec![0; m];
        let mut redge_idx = vec![0; m];

        let mut g = {
            let mut degree = vec![0; self.n];
            let mut elist: Vec<(usize, InnerEdge)> = vec![];
            elist.reserve(2 * m);
            for i in 0..m {
                let e = &self.edges[i];
                edge_idx[i] = degree[e.from];
                redge_idx[i] = degree[e.to];
                degree[e.from] += 1;
                degree[e.to] += 1;
                elist.push((
                    e.from,
                    InnerEdge {
                        to: e.to,
                        rev: 0,
                        cap: e.cap - e.flow,
                        cost: e.cost,
                    },
                ));
                elist.push((
                    e.to,
                    InnerEdge {
                        to: e.from,
                        rev: 0,
                        cap: e.flow,
                        cost: -e.cost,
                    },
                ));
            }
            let mut g = Csr::new(self.n, &elist);
            for i in 0..m {
                let e = &self.edges[i];
                edge_idx[i] += g.start[e.from];
                redge_idx[i] += g.start[e.to];
                g.elist[edge_idx[i]].rev = redge_idx[i];
                g.elist[redge_idx[i]].rev = edge_idx[i];
            }
            g
        };

        struct Env {
            dual: Vec<i64>,
            dist: Vec<i64>,
            prev_e: Vec<usize>,
            vis: Vec<bool>,
            que: BinaryHeap<Reverse<(i64, usize)>>,
            que_min: Vec<usize>,
        }

        let mut env = Env {
            dual: vec![0; self.n],
            dist: vec![0; self.n],
            prev_e: vec![0; self.n],
            vis: vec![false; self.n],
            que: BinaryHeap::new(),
            que_min: vec![],
        };

        fn dual_ref(s: usize, t: usize, g: &Csr<InnerEdge>, env: &mut Env) -> bool {
            for i in 0..env.dist.len() {
                env.dist[i] = i64::MAX;
            }
            env.vis.fill(false);
            env.que_min.clear();
            env.que.clear();

            env.dist[s] = 0;
            env.que_min.push(s);

            while !env.que_min.is_empty() || !env.que.is_empty() {
                let v = if let Some(v) = env.que_min.pop() {
                    v
                } else {
                    let Reverse((_, v)) = env.que.pop().unwrap();
                    v
                };
                if env.vis[v] {
                    continue;
                }
                env.vis[v] = true;
                if v == t {
                    break;
                }

                for i in g.start[v]..g.start[v + 1] {
                    let e = g.elist[i];
                    if e.cap <= 0 {
                        continue;
                    }
                    let cost = e.cost - env.dual[e.to] + env.dual[v];
                    if env.dist[v] + cost < env.dist[e.to] {
                        let dist_to = env.dist[v] + cost;
                        env.dist[e.to] = dist_to;
                        env.prev_e[e.to] = e.rev;
                        if dist_to == env.dist[v] {
                            env.que_min.push(e.to);
                        } else {
                            env.que.push(Reverse((dist_to, e.to)));
                        }
                    }
                }
            }

            if !env.vis[t] {
                return false;
            }

            for v in 0..env.dual.len() {
                if !env.vis[v] {
                    continue;
                }
                // dual[v] = dual[v] - dist[t] + dist[v]
                //         = dual[v] - (shortest(s, t) + dual[s] - dual[t]) + (shortest(s, v) + dual[s] - dual[v])
                //         = shortest(s, v) - shortest(s, t) + dual[t]
                //         = shortest(s, v) - shortest(s, t) >= 0 - (n-1)C
                env.dual[v] -= env.dist[t] - env.dist[v];
            }
            true
        }

        let result = {
            let mut flow = 0;
            let mut cost = 0;
            let mut prev_cost_per_flow: Option<i64> = None;
            let mut result = vec![(0, 0)];

            while flow < flow_limit {
                if !dual_ref(s, t, &g, &mut env) {
                    break;
                }
                let mut c = flow_limit - flow;
                let mut v = t;
                while v != s {
                    let rev = env.prev_e[v];
                    c = std::cmp::min(c, g.elist[g.elist[rev].rev].cap);
                    v = g.elist[rev].to;
                }
                let mut v = t;
                while v != s {
                    let rev = env.prev_e[v];
                    let fwd = g.elist[rev].rev;
                    g.elist[rev].cap += c;
                    g.elist[fwd].cap -= c;
                    v = g.elist[rev].to;
                }
                let d = -env.dual[s];
                flow += c;
                cost += c * d;
                if prev_cost_per_flow == Some(d) {
                    result.pop();
                }
                result.push((flow, cost));
                prev_cost_per_flow = Some(d);
            }
            result
        };

        for i in 0..m {
            self.edges[i].flow = self.edges[i].cap - g.elist[edge_idx[i]].cap;
        }

        result
    }
}
