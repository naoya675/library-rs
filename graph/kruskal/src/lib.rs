use union_find::UnionFind;

pub type Cost = i64;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: Cost,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: Cost) -> Self {
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

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn minimum_spanning_tree(&mut self) -> (Cost, Vec<Edge>) {
        self.edge.sort_by(|a, b| a.cost.cmp(&b.cost));
        let mut uf = UnionFind::new(self.size);
        let mut res = 0;
        let mut res_edge = vec![];
        for edge in &self.edge {
            if uf.merge(edge.to, edge.from) {
                res += edge.cost;
                res_edge.push(edge.clone());
            }
        }
        (res, res_edge)
    }

    pub fn maximum_spanning_tree(&mut self) -> (Cost, Vec<Edge>) {
        self.edge.sort_by(|a, b| b.cost.cmp(&a.cost));
        let mut uf = UnionFind::new(self.size);
        let mut res = 0;
        let mut res_edge = vec![];
        for edge in &self.edge {
            if uf.merge(edge.to, edge.from) {
                res += edge.cost;
                res_edge.push(edge.clone());
            }
        }
        (res, res_edge)
    }
}

pub fn minimum_spanning_tree(size: usize, edge: &mut Vec<(usize, usize, i64)>) -> (Cost, Vec<(usize, usize, i64)>) {
    edge.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFind::new(size);
    let mut res = 0;
    let mut res_edge = vec![];
    for &mut (from, to, cost) in edge {
        if uf.merge(to, from) {
            res += cost;
            res_edge.push((from, to, cost));
        }
    }
    (res, res_edge)
}

pub fn maximum_spanning_tree(size: usize, edge: &mut Vec<(usize, usize, i64)>) -> (Cost, Vec<(usize, usize, i64)>) {
    edge.sort_by(|a, b| b.2.cmp(&a.2));
    let mut uf = UnionFind::new(size);
    let mut res = 0;
    let mut res_edge = vec![];
    for &mut (from, to, cost) in edge {
        if uf.merge(to, from) {
            res += cost;
            res_edge.push((from, to, cost));
        }
    }
    (res, res_edge)
}
