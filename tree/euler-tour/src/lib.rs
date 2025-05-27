use segment_tree::SegmentTree;

#[derive(Debug, Clone)]
pub struct Edge<Cost> {
    from: usize,
    to: usize,
    cost: Cost,
}

impl<Cost: Copy> Edge<Cost> {
    pub fn new(from: usize, to: usize, cost: Cost) -> Self {
        Self { from, to, cost }
    }
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

impl<Cost: Copy + Default> EulerTour<Cost> {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            depth: vec![0; n],
            preorder: vec![0; n],
            postorder: vec![0; n],
            rmq: SegmentTree::new(n + n, |a, b| if a.0 < b.0 { a } else { b }, (usize::MAX, n)),
            n,
            time: 0,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.graph[from].push(Edge::new(from, to, cost));
    }

    pub fn init(&mut self, root: usize) {
        self.time = 0;
        self.dfs(root, self.n);
    }

    fn dfs(&mut self, v: usize, p: usize) {
        self.rmq.set(self.time, (self.depth[v], v));
        self.preorder[v] = self.time;
        self.time += 1;
        for edge in self.graph[v].clone() {
            if edge.to == p {
                continue;
            }
            self.depth[edge.to] = self.depth[v] + 1;
            self.dfs(edge.to, v);
        }
        if p == self.n {
            return;
        }
        self.rmq.set(self.time, (self.depth[v] - 1, p));
        self.postorder[v] = self.time;
        self.time += 1;
    }

    // Lowest Common Ancestor
    pub fn lca(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let l = std::cmp::min(self.preorder[a], self.preorder[b]);
        let r = std::cmp::max(self.preorder[a], self.preorder[b]) + 1;
        self.rmq.prod(l, r).1
    }
}
