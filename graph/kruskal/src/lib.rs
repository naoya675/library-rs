use union_find::UnionFind;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: i64) -> Self {
        Self { from, to, cost }
    }
}

#[derive(Debug, Clone)]
pub struct Kruskal {
    size: usize,
    edge: Vec<Edge>,
}

impl Kruskal {
    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn minimum_spanning_tree(&mut self) -> (i64, Vec<Edge>) {
        self.edge.sort_by(|a, b| a.cost.cmp(&b.cost));
        let mut uf = UnionFind::new(self.size);
        let mut res = 0;
        let mut res_edge = vec![];
        for edge in &self.edge {
            if uf.same(edge.from, edge.to) {
                continue;
            }
            uf.merge(edge.from, edge.to);
            res += edge.cost;
            res_edge.push(edge.clone());
        }
        (res, res_edge)
    }

    pub fn maximum_spanning_tree(&mut self) -> (i64, Vec<Edge>) {
        self.edge.sort_by(|a, b| b.cost.cmp(&a.cost));
        let mut uf = UnionFind::new(self.size);
        let mut res = 0;
        let mut res_edge = vec![];
        for edge in &self.edge {
            if uf.same(edge.from, edge.to) {
                continue;
            }
            uf.merge(edge.from, edge.to);
            res += edge.cost;
            res_edge.push(edge.clone());
        }
        (res, res_edge)
    }
}

pub fn minimum_spanning_tree(size: usize, edge: &mut Vec<(usize, usize, i64)>) -> (i64, Vec<(usize, usize, i64)>) {
    edge.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFind::new(size);
    let mut res = 0;
    let mut res_edge = vec![];
    for &mut (from, to, cost) in edge {
        if uf.same(from, to) {
            continue;
        }
        uf.merge(from, to);
        res += cost;
        res_edge.push((from, to, cost));
    }
    (res, res_edge)
}

pub fn maximum_spanning_tree(size: usize, edge: &mut Vec<(usize, usize, i64)>) -> (i64, Vec<(usize, usize, i64)>) {
    edge.sort_by(|a, b| b.2.cmp(&a.2));
    let mut uf = UnionFind::new(size);
    let mut res = 0;
    let mut res_edge = vec![];
    for &mut (from, to, cost) in edge {
        if uf.same(from, to) {
            continue;
        }
        uf.merge(from, to);
        res += cost;
        res_edge.push((from, to, cost));
    }
    (res, res_edge)
}
