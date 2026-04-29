#[derive(Debug, Clone)]
pub struct UnionFindWithRollback {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
    history: Vec<Option<usize>>,
}

impl UnionFindWithRollback {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            par: (0..n).collect(),
            siz: vec![1; n],
            history: vec![],
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            self.history.push(None);
            return x;
        }
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.siz[x] += self.siz[y];
        self.par[y] = x;
        self.history.push(Some(y));
        x
    }

    pub fn rollback(&mut self) {
        if let Some(Some(y)) = self.history.pop() {
            let x = self.par[y];
            self.siz[x] -= self.siz[y];
            self.par[y] = y;
        }
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        self.leader(x) == self.leader(y)
    }

    pub fn leader(&self, x: usize) -> usize {
        assert!(x < self.n);
        if self.par[x] == x {
            return x;
        }
        self.leader(self.par[x])
    }

    pub fn size(&self, x: usize) -> usize {
        assert!(x < self.n);
        let x = self.leader(x);
        self.siz[x]
    }

    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[self.leader(i)].push(i);
        }
        res.into_iter().filter(|f| !f.is_empty()).collect()
    }
}
