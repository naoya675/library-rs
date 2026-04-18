use segment_tree::SegmentTree;

#[derive(Debug, Clone, Copy)]
pub struct Edge<Cost> {
    from: usize,
    to: usize,
    cost: Cost,
}

#[derive(Debug, Clone)]
pub struct EulerTour<Cost> {
    graph: Vec<Vec<Edge<Cost>>>,
    depth: Vec<usize>,
    preorder: Vec<usize>,
    postorder: Vec<usize>,
    rmq: SegmentTree<(usize, usize)>,
    n: usize,
    time: usize,
}

impl<Cost: Copy> EulerTour<Cost> {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            depth: vec![0; n],
            preorder: vec![0; n],
            postorder: vec![0; n],
            rmq: SegmentTree::new(n + n, |x, y| std::cmp::min(x, y), (usize::MAX, n)),
            n,
            time: 0,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.graph[from].push(Edge { from, to, cost });
    }

    pub fn init(&mut self, root: usize) {
        self.time = 0;
        self.dfs(root, None);
    }

    fn dfs(&mut self, v: usize, p: Option<usize>) {
        self.rmq.set(self.time, (self.depth[v], v));
        self.preorder[v] = self.time;
        self.time += 1;
        for j in 0..self.graph[v].len() {
            let edge = self.graph[v][j];
            if Some(edge.to) == p {
                continue;
            }
            self.depth[edge.to] = self.depth[v] + 1;
            self.dfs(edge.to, Some(v));
        }
        if let Some(p) = p {
            self.rmq.set(self.time, (self.depth[v] - 1, p));
        }
        self.postorder[v] = self.time;
        self.time += 1;
    }

    // Lowest Common Ancestor
    pub fn lca(&self, u: usize, v: usize) -> usize {
        assert!(u < self.n);
        assert!(v < self.n);
        let l = std::cmp::min(self.preorder[u], self.preorder[v]);
        let r = std::cmp::max(self.preorder[u], self.preorder[v]) + 1;
        self.rmq.prod(l, r).1
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

    pub fn for_each<F>(&self, u: usize, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        f(self.preorder[l], self.preorder[u] + 1);
        f(self.preorder[l] + 1, self.preorder[v] + 1);
    }

    pub fn for_each_edge<F>(&self, u: usize, v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        f(self.preorder[l] + 1, self.preorder[u] + 1);
        f(self.preorder[l] + 1, self.preorder[v] + 1);
    }

    pub fn for_each_with<F, G>(&self, u: usize, v: usize, mut f: F, mut g: G)
    where
        F: FnMut(usize, usize),
        G: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        g(self.preorder[l], self.preorder[u] + 1);
        f(self.preorder[l] + 1, self.preorder[v] + 1);
    }

    pub fn for_each_edge_with<F, G>(&self, u: usize, v: usize, mut f: F, mut g: G)
    where
        F: FnMut(usize, usize),
        G: FnMut(usize, usize),
    {
        let l = self.lca(u, v);
        g(self.preorder[l] + 1, self.preorder[u] + 1);
        f(self.preorder[l] + 1, self.preorder[v] + 1);
    }

    pub fn index(&self, v: usize) -> (usize, usize) {
        assert!(v < self.n);
        (self.preorder[v], self.postorder[v])
    }
}
