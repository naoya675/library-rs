use compressed_sparse_row::Csr;

#[derive(Debug, Clone, Copy, Default)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
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
        self.edge.push((from, Edge { from: from, to: to }));
    }

    // Tarjan's strongly connected components algorithm
    pub fn scc_ids(&mut self) -> (usize, Vec<usize>) {
        let g: Csr<Edge> = Csr::new(self.size, &self.edge);

        struct Env {
            size: usize,
            group_num: usize,
            now_ord: usize,
            used: Vec<bool>,
            stack: Vec<usize>,
            low: Vec<usize>, // low link
            ord: Vec<usize>, // dfs order
            ids: Vec<usize>,
        }

        let mut env = Env {
            size: self.size,
            group_num: 0,
            now_ord: 0,
            used: vec![false; self.size],
            stack: Vec::with_capacity(self.size),
            low: vec![0; self.size],
            ord: vec![0; self.size],
            ids: vec![0; self.size],
        };

        // fn dfs(v: usize, env: &mut Env) {}

        struct Recursive<'a> {
            f: &'a dyn Fn(&Recursive<'a>, &mut Env, usize),
        }

        let dfs = Recursive {
            f: &|dfs: &Recursive<'_>, env: &mut Env, v: usize| {
                env.used[v] = true;
                env.low[v] = env.now_ord;
                env.ord[v] = env.now_ord;
                env.now_ord += 1;
                env.stack.push(v);
                for &edge in &g[v] {
                    if !env.used[edge.to] {
                        (dfs.f)(dfs, env, edge.to);
                        env.low[v] = env.low[v].min(env.low[edge.to]);
                    } else {
                        env.low[v] = env.low[v].min(env.ord[edge.to]);
                    }
                }
                if env.low[v] == env.ord[v] {
                    loop {
                        let u = env.stack.pop().unwrap();
                        env.ord[u] = env.size;
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
            if !env.used[i] {
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
