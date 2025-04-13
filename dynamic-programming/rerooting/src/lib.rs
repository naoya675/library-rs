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
pub struct Rerooting<
    Cost,
    Data,
    Merge: Fn(Data, Data) -> Data,
    E: Fn() -> Data,
    Leaf: Fn() -> Data,
    Apply: Fn(Data, usize, usize, Cost) -> Data,
> {
    graph: Vec<Vec<Edge<Cost>>>,
    dp: Vec<Data>,
    memo: Vec<Data>,
    merge: Merge,
    e: E,
    leaf: Leaf,
    apply: Apply,
    n: usize,
}

impl<
        Cost: Copy + Default,
        Data: Copy,
        Merge: Fn(Data, Data) -> Data,
        E: Fn() -> Data,
        Leaf: Fn() -> Data,
        Apply: Fn(Data, usize, usize, Cost) -> Data,
    > Rerooting<Cost, Data, Merge, E, Leaf, Apply>
{
    pub fn new(n: usize, merge: Merge, e: E, leaf: Leaf, apply: Apply) -> Self {
        Self {
            graph: vec![vec![]; n],
            dp: vec![],
            memo: vec![],
            merge,
            e,
            leaf,
            apply,
            n,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.graph[from].push(Edge::new(from, to, cost));
    }

    pub fn run(&mut self) -> Vec<Data> {
        self.memo.resize(self.n, (self.e)());
        self.dp.resize(self.n, (self.e)());
        self.dfs1(0, usize::MAX);
        self.dfs2(0, usize::MAX, (self.e)());
        self.dp.clone()
    }

    fn dfs1(&mut self, c: usize, p: usize) {
        let mut upd = false;
        for edge in self.graph[c].clone() {
            if edge.to == p {
                continue;
            }
            self.dfs1(edge.to, c);
            upd = true;
            self.memo[c] = (self.merge)(
                self.memo[c],
                (self.apply)(self.memo[edge.to], edge.to, c, edge.cost),
            );
        }
        if !upd {
            self.memo[c] = (self.leaf)();
        }
    }

    fn dfs2(&mut self, c: usize, p: usize, val: Data) {
        let mut ds = vec![val];
        for edge in self.graph[c].clone() {
            if edge.to == p {
                continue;
            }
            ds.push((self.apply)(self.memo[edge.to], edge.to, c, edge.cost));
        }
        let n = ds.len();
        let mut idx = 1;
        let mut head = vec![(self.e)(); n + 1];
        let mut tail = vec![(self.e)(); n + 1];
        for i in 0..n {
            head[i + 1] = (self.merge)(head[i], ds[i]);
        }
        for i in (0..n).rev() {
            tail[i] = (self.merge)(tail[i + 1], ds[i]);
        }
        self.dp[c] = head[n];
        for edge in self.graph[c].clone() {
            if edge.to == p {
                continue;
            }
            let sub = (self.merge)(head[idx], tail[idx + 1]);
            self.dfs2(edge.to, c, (self.apply)(sub, c, edge.to, edge.cost));
            idx += 1;
        }
    }
}

pub struct RerootingDiameter;

impl RerootingDiameter {
    pub fn new(
        n: usize,
    ) -> Rerooting<
        usize,
        usize,
        impl Fn(usize, usize) -> usize,
        impl Fn() -> usize,
        impl Fn() -> usize,
        impl Fn(usize, usize, usize, usize) -> usize,
    > {
        let merge = |a: usize, b: usize| std::cmp::max(a, b);
        let e = || 0;
        let leaf = || 0;
        let apply = |a: usize, _: usize, _: usize, w: usize| a + w;
        Rerooting::new(n, merge, e, leaf, apply)
    }
}

// reference: https://atcoder.jp/contests/abc222/editorial/2749
