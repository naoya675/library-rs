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
pub struct CompressedSparseRow {
    start: Vec<usize>,
    elist: Vec<Edge>,
}

impl CompressedSparseRow {
    pub fn new(n: usize, edges: &[(usize, Edge)]) -> Self {
        let mut start = vec![0; n + 1];
        let mut elist = vec![Edge { from: 0, to: 0 }; edges.len()];
        for &(from, _) in edges {
            start[from + 1] += 1;
        }
        for i in 1..=n {
            start[i] += start[i - 1];
        }
        let mut counter = start.clone();
        for &(from, e) in edges {
            elist[counter[from]] = e;
            counter[from] += 1;
        }
        Self { start, elist }
    }
}

#[derive(Debug, Clone)]
pub struct StronglyConnectedComponents {
    size: usize,
    edge: Vec<(usize, Edge)>,
}

impl StronglyConnectedComponents {
    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.size);
        assert!(to < self.size);
        self.edge.push((from, Edge::new(from, to)));
    }

    // Tarjan's strongly connected components algorithm
    pub fn scc_ids(&mut self) -> (usize, Vec<usize>) {
        let g = CompressedSparseRow::new(self.size, &self.edge);

        struct Env {
            group_num: usize,
            now_ord: usize,
            visited: Vec<usize>,
            low: Vec<usize>, // lowlink
            ord: Vec<usize>, // dfs order
            ids: Vec<usize>,
        }

        let mut env = Env {
            group_num: 0,
            now_ord: 0,
            visited: Vec::with_capacity(self.size),
            low: vec![0; self.size],
            ord: vec![usize::MAX; self.size],
            ids: vec![0; self.size],
        };

        // fn dfs(v: usize, env: &mut Env) {}

        struct Recursive<'a> {
            f: &'a dyn Fn(&Recursive<'a>, &mut Env, usize),
        }

        let dfs = Recursive {
            f: &|dfs: &Recursive<'_>, env: &mut Env, v: usize| {
                env.low[v] = env.now_ord;
                env.ord[v] = env.now_ord;
                env.now_ord += 1;
                env.visited.push(v);
                for i in g.start[v]..g.start[v + 1] {
                    let to = g.elist[i].to;
                    if env.ord[to] == usize::MAX {
                        (dfs.f)(dfs, env, to);
                        env.low[v] = env.low[v].min(env.low[to]);
                    } else {
                        env.low[v] = env.low[v].min(env.ord[to]);
                    }
                }
                if env.low[v] == env.ord[v] {
                    loop {
                        let u = env.visited.pop().unwrap();
                        env.ord[u] = self.size;
                        env.ids[u] = env.group_num;
                        if u == v {
                            break;
                        }
                    }
                    env.group_num += 1;
                }
            },
        };

        for i in 0..self.size {
            if env.ord[i] == usize::MAX {
                (dfs.f)(&dfs, &mut env, i);
            }
        }
        for x in env.ids.iter_mut() {
            *x = env.group_num - 1 - *x;
        }
        (env.group_num, env.ids)
    }

    pub fn scc(&mut self) -> Vec<Vec<usize>> {
        let (group_num, ids) = self.scc_ids();
        let mut counts = vec![0; group_num];
        for &i in &ids {
            counts[i] += 1;
        }
        let mut groups = vec![vec![]; group_num];
        for i in 0..group_num {
            groups[i].reserve(counts[i]);
        }
        for i in 0..self.size {
            groups[ids[i]].push(i);
        }
        groups
    }
}
