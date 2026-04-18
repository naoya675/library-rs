#[derive(Debug, Clone, Copy)]
pub struct Edge<Cost> {
    from: usize,
    to: usize,
    cost: Cost,
}

#[derive(Debug, Clone)]
pub struct Rerooting<Cost, Data, Merge: Fn(Data, Data) -> Data, E: Fn() -> Data, Leaf: Fn(usize) -> Data, Apply: Fn(Data, usize, usize, Cost) -> Data> {
    graph: Vec<Vec<Edge<Cost>>>,
    dp: Vec<Data>,
    memo: Vec<Data>,
    merge: Merge,
    e: E,
    leaf: Leaf,
    apply: Apply,
    n: usize,
}

impl<Cost: Copy, Data: Copy, Merge: Fn(Data, Data) -> Data, E: Fn() -> Data, Leaf: Fn(usize) -> Data, Apply: Fn(Data, usize, usize, Cost) -> Data>
    Rerooting<Cost, Data, Merge, E, Leaf, Apply>
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
        self.graph[from].push(Edge { from, to, cost });
    }

    pub fn run(&mut self) -> Vec<Data> {
        self.memo.resize(self.n, (self.e)());
        self.dp.resize(self.n, (self.e)());
        self.dfs1(0, None);
        self.dfs2(0, None, (self.e)());
        self.dp.clone()
    }

    fn dfs1(&mut self, c: usize, p: Option<usize>) {
        let mut upd = false;
        for j in 0..self.graph[c].len() {
            let edge = self.graph[c][j];
            if Some(edge.to) == p {
                continue;
            }
            self.dfs1(edge.to, Some(c));
            upd = true;
            self.memo[c] = (self.merge)(self.memo[c], (self.apply)(self.memo[edge.to], edge.to, c, edge.cost));
        }
        if !upd {
            self.memo[c] = (self.leaf)(c);
        }
    }

    fn dfs2(&mut self, c: usize, p: Option<usize>, val: Data) {
        let mut ds = vec![(self.e)()];
        for j in 0..self.graph[c].len() {
            let edge = self.graph[c][j];
            if Some(edge.to) == p {
                ds.push((self.apply)(val, edge.to, c, edge.cost));
            } else {
                ds.push((self.apply)(self.memo[edge.to], edge.to, c, edge.cost));
            }
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
        for j in 0..self.graph[c].len() {
            let edge = self.graph[c][j];
            if Some(edge.to) == p {
                idx += 1;
                continue;
            }
            self.dfs2(edge.to, Some(c), (self.merge)(head[idx], tail[idx + 1]));
            idx += 1;
        }
    }

    /*
     * Symmetric
    fn dfs2(&mut self, c: usize, p: Option<usize>, val: Data) {
        let mut ds = vec![val];
        for j in 0..self.graph[c].len() {
            let edge = self.graph[c][j];
            if Some(edge.to) == p {
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
        for j in 0..self.graph[c].len() {
            let edge = self.graph[c][j];
            if Some(edge.to) == p {
                continue;
            }
            let sub = (self.merge)(head[idx], tail[idx + 1]);
            self.dfs2(edge.to, Some(c), (self.apply)(sub, c, edge.to, edge.cost));
            idx += 1;
        }
    }
     */
}
