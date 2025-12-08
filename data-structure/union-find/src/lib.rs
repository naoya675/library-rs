#[derive(Debug, Clone)]
pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            par: (0..n).collect::<Vec<usize>>(),
            siz: vec![1; n],
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return x;
        }
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.siz[x] += self.siz[y];
        self.par[y] = x;
        x
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
        self.par[x] = leader;
        self.par[x]
    }

    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x = self.leader(x);
        self.siz[x]
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[self.leader(i)].push(i);
        }
        res.into_iter().filter(|f| !f.is_empty()).collect::<Vec<_>>()
    }
}
