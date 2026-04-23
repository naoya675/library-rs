use compressed_sparse_row::Csr;

#[derive(Debug, Clone, Copy)]
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
    pub fn scc_ids(&self) -> (usize, Vec<usize>) {
        struct Env {
            g: Csr<Edge>,
            size: usize,
            group_num: usize,
            now_ord: usize,
            stack: Vec<usize>,
            low: Vec<usize>,
            ord: Vec<Option<usize>>,
            ids: Vec<usize>,
        }

        let mut env = Env {
            g: Csr::new(self.size, &self.edge),
            size: self.size,
            group_num: 0,
            now_ord: 0,
            stack: Vec::with_capacity(self.size),
            low: vec![0; self.size],
            ord: vec![None; self.size],
            ids: vec![0; self.size],
        };

        // fn dfs(v: usize, env: &mut Env) {}

        struct Recursive<'a> {
            f: &'a dyn Fn(&Recursive<'a>, &mut Env, usize),
        }

        let dfs = Recursive {
            f: &|dfs: &Recursive<'_>, env: &mut Env, v: usize| {
                env.low[v] = env.now_ord;
                env.ord[v] = Some(env.now_ord);
                env.now_ord += 1;
                env.stack.push(v);
                for i in 0..env.g[v].len() {
                    let edge = env.g[v][i];
                    if let Some(x) = env.ord[edge.to] {
                        env.low[v] = env.low[v].min(x);
                    } else {
                        (dfs.f)(dfs, env, edge.to);
                        env.low[v] = env.low[v].min(env.low[edge.to]);
                    }
                }
                if env.low[v] == env.ord[v].unwrap() {
                    loop {
                        let u = env.stack.pop().unwrap();
                        env.ord[u] = Some(env.size);
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
            if env.ord[i].is_none() {
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
