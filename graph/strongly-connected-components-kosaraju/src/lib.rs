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
    pub fn new(n: usize, edges: &Vec<(usize, Edge)>) -> Self {
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
    fedge: Vec<(usize, Edge)>,
    redge: Vec<(usize, Edge)>,
}

impl StronglyConnectedComponents {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            fedge: vec![],
            redge: vec![],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.size);
        assert!(to < self.size);
        self.fedge.push((from, Edge::new(from, to)));
        self.redge.push((to, Edge::new(to, from)));
    }

    // Kosaraju's strongly connected components algorithm
    pub fn scc_ids(&mut self) -> (usize, Vec<usize>) {
        let fg = CompressedSparseRow::new(self.size, &self.fedge);
        let rg = CompressedSparseRow::new(self.size, &self.redge);

        struct Env {
            group_num: usize,
            visited: Vec<bool>,
            ord: Vec<usize>,
            ids: Vec<usize>,
        }

        let mut env = Env {
            group_num: 0,
            visited: vec![false; self.size],
            ord: Vec::with_capacity(self.size),
            ids: vec![0; self.size],
        };

        struct Recursive<'a> {
            f: &'a dyn Fn(&Recursive<'a>, &mut Env, usize),
        }

        let dfs1 = Recursive {
            f: &|dfs: &Recursive<'_>, env: &mut Env, v: usize| {
                env.visited[v] = true;
                for i in fg.start[v]..fg.start[v + 1] {
                    let to = fg.elist[i].to;
                    if !env.visited[to] {
                        (dfs.f)(dfs, env, to);
                    }
                }
                env.ord.push(v);
            },
        };

        let dfs2 = Recursive {
            f: &|dfs: &Recursive<'_>, env: &mut Env, v: usize| {
                env.ids[v] = env.group_num;
                env.visited[v] = true;
                for i in rg.start[v]..rg.start[v + 1] {
                    let to = rg.elist[i].to;
                    if !env.visited[to] {
                        (dfs.f)(dfs, env, to);
                    }
                }
            },
        };

        for i in 0..self.size {
            if !env.visited[i] {
                (dfs1.f)(&dfs1, &mut env, i);
            }
        }
        env.visited.fill(false);
        for i in (0..self.size).rev() {
            let v = env.ord[i];
            if !env.visited[v] {
                (dfs2.f)(&dfs2, &mut env, v);
                env.group_num += 1;
            }
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
