#[derive(Debug, Clone, Copy)]
pub struct Edge<Cost> {
    from: usize,
    to: usize,
    cost: Cost,
}

#[derive(Debug, Clone)]
pub struct HeavyLightDecomposition<Cost> {
    graph: Vec<Vec<Edge<Cost>>>,
    depth: Vec<usize>,
    subtree: Vec<usize>,
    head: Vec<usize>,
    parent: Vec<usize>,
    preorder: Vec<usize>,
    postorder: Vec<usize>,
    rev: Vec<usize>,
    n: usize,
    time: usize,
}

impl<Cost: Copy> HeavyLightDecomposition<Cost> {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            depth: vec![0; n],
            subtree: vec![0; n],
            head: vec![0; n],
            parent: vec![0; n],
            preorder: vec![0; n],
            postorder: vec![0; n],
            rev: vec![0; n],
            n,
            time: 0,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.graph[from].push(Edge { from, to, cost });
    }

    pub fn init(&mut self, root: usize) {
        self.dfs1(root, None);
        self.time = 0;
        self.head[root] = root;
        self.dfs2(root, None);
    }

    fn dfs1(&mut self, v: usize, p: Option<usize>) {
        self.subtree[v] = 1;
        if !self.graph[v].is_empty() && Some(self.graph[v][0].to) == p {
            let last = self.graph[v].len() - 1;
            self.graph[v].swap(0, last);
        }
        for i in 0..self.graph[v].len() {
            let edge = self.graph[v][i];
            if Some(edge.to) == p {
                continue;
            }
            self.depth[edge.to] = self.depth[v] + 1;
            self.dfs1(edge.to, Some(v));
            self.subtree[v] += self.subtree[edge.to];
            if self.subtree[self.graph[v][0].to] < self.subtree[edge.to] {
                self.graph[v].swap(0, i);
            }
        }
    }

    fn dfs2(&mut self, v: usize, p: Option<usize>) {
        self.preorder[v] = self.time;
        self.time += 1;
        self.rev[self.preorder[v]] = v;
        for i in 0..self.graph[v].len() {
            let edge = self.graph[v][i];
            if Some(edge.to) == p {
                continue;
            }
            self.parent[edge.to] = v;
            self.head[edge.to] = if edge.to == self.graph[v][0].to { self.head[v] } else { edge.to };
            self.dfs2(edge.to, Some(v));
        }
        self.postorder[v] = self.time; // self.postorder[v] == preorder[v] + subtree[v]
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
        self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u, v)]
    }

    // Level Ancestor
    pub fn la(&self, mut v: usize, mut k: usize) -> usize {
        assert!(v < self.n);
        assert!(k <= self.depth[v]);
        loop {
            let u = self.head[v];
            if self.preorder[v] - k >= self.preorder[u] {
                return self.rev[self.preorder[v] - k];
            }
            k -= self.preorder[v] - self.preorder[u] + 1;
            v = self.parent[u];
        }
    }

    // Lowest Common Ancestor
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        assert!(u < self.n);
        assert!(v < self.n);
        loop {
            if self.preorder[u] > self.preorder[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.parent[self.head[v]];
        }
    }

    pub fn for_each_subtree<F>(&self, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        assert!(v < self.n);
        f(self.preorder[v], self.postorder[v]);
    }

    pub fn for_each_subtree_edge<F>(&self, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        assert!(v < self.n);
        f(self.preorder[v] + 1, self.postorder[v]);
    }

    // Noncommutative
    pub fn for_each_noncommutative<F>(&self, u: usize, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        for (l, r) in self.ascend(u, l) {
            f(l + 1, r);
        }
        f(self.preorder[l], self.preorder[l] + 1);
        for (l, r) in self.descend(l, v) {
            f(l, r + 1);
        }
    }

    // Noncommutative
    pub fn for_each_noncommutative_edge<F>(&self, u: usize, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        for (l, r) in self.ascend(u, l) {
            f(l + 1, r);
        }
        for (l, r) in self.descend(l, v) {
            f(l, r + 1);
        }
    }

    pub fn for_each<F>(&self, u: usize, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        for (l, r) in self.ascend(u, l) {
            f(r.min(l + 1), r.max(l + 1));
        }
        f(self.preorder[l], self.preorder[l] + 1);
        for (l, r) in self.descend(l, v) {
            f(l.min(r + 1), l.max(r + 1));
        }
    }

    pub fn for_each_edge<F>(&self, u: usize, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        for (l, r) in self.ascend(u, l) {
            f(r.min(l + 1), r.max(l + 1));
        }
        for (l, r) in self.descend(l, v) {
            f(l.min(r + 1), l.max(r + 1));
        }
    }

    pub fn index(&self, v: usize) -> (usize, usize) {
        assert!(v < self.n);
        (self.preorder[v], self.postorder[v])
    }

    // [u, v)
    fn ascend(&self, mut u: usize, v: usize) -> Vec<(usize, usize)> {
        assert!(self.lca(u, v) == v);
        let mut res = vec![];
        loop {
            if self.head[u] != self.head[v] {
                res.push((self.preorder[u], self.preorder[self.head[u]]));
                u = self.parent[self.head[u]];
            } else {
                break;
            }
        }
        if u != v {
            res.push((self.preorder[u], self.preorder[v] + 1));
        }
        res
    }

    // (u, v]
    fn descend(&self, u: usize, v: usize) -> Vec<(usize, usize)> {
        assert!(self.lca(u, v) == u);
        if u == v {
            return vec![];
        }
        if self.head[u] == self.head[v] {
            return vec![(self.preorder[u] + 1, self.preorder[v])];
        }
        let mut res = self.descend(u, self.parent[self.head[v]]);
        res.push((self.preorder[self.head[v]], self.preorder[v]));
        res
    }
}
