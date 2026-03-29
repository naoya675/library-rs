#[derive(Debug, Clone)]
pub struct BipartiteUnionFind {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
    parity: Vec<u8>,
    parity_count: Vec<[usize; 2]>,
    bipartite: Vec<bool>,
}

impl BipartiteUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            par: (0..n).collect(),
            siz: vec![1; n],
            parity: vec![0; n],
            parity_count: vec![[1, 0]; n],
            bipartite: vec![true; n],
        }
    }

    pub fn merge_with_parity(&mut self, x: usize, y: usize, w: u8) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
        assert!(w <= 1);
        let mut rx = self.leader(x);
        let mut ry = self.leader(y);
        let px = self.parity[x];
        let py = self.parity[y];
        if rx == ry {
            if (px ^ py) != w {
                self.bipartite[rx] = false;
            }
            return rx;
        }
        let edge_w = px ^ py ^ w;
        if self.siz[rx] < self.siz[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        self.siz[rx] += self.siz[ry];
        self.par[ry] = rx;
        self.parity[ry] = edge_w;
        if edge_w == 0 {
            self.parity_count[rx][0] += self.parity_count[ry][0];
            self.parity_count[rx][1] += self.parity_count[ry][1];
        } else {
            self.parity_count[rx][0] += self.parity_count[ry][1];
            self.parity_count[rx][1] += self.parity_count[ry][0];
        }
        if !self.bipartite[ry] {
            self.bipartite[rx] = false;
        }
        rx
    }

    pub fn merge(&mut self, x: usize, y: usize) -> usize {
        self.merge_with_parity(x, y, 1)
    }

    pub fn merge_same(&mut self, x: usize, y: usize) -> usize {
        self.merge_with_parity(x, y, 0)
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        self.leader(x) == self.leader(y)
    }

    pub fn leader(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.par[x] == x {
            return x;
        }
        let leader = self.leader(self.par[x]);
        self.parity[x] ^= self.parity[self.par[x]];
        self.par[x] = leader;
        self.par[x]
    }

    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x = self.leader(x);
        self.siz[x]
    }

    pub fn parity(&mut self, x: usize) -> u8 {
        assert!(x < self.n);
        self.leader(x);
        self.parity[x]
    }

    pub fn same_parity(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        assert!(self.same(x, y));
        self.parity(x) == self.parity(y)
    }

    pub fn parity_count(&mut self, x: usize) -> (usize, usize) {
        assert!(x < self.n);
        let x = self.leader(x);
        (self.parity_count[x][0], self.parity_count[x][1])
    }

    pub fn is_bipartite_component(&mut self, x: usize) -> bool {
        assert!(x < self.n);
        let x = self.leader(x);
        self.bipartite[x]
    }

    pub fn is_bipartite(&self) -> bool {
        self.bipartite.iter().all(|&b| b)
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[self.leader(i)].push(i);
        }
        res.into_iter().filter(|f| !f.is_empty()).collect()
    }
}
