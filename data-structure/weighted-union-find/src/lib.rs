use internal_type::Zero;

#[derive(Debug, Clone)]
pub struct WeightedUnionFind<T> {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
    diff_weight: Vec<T>,
}

impl<T: Copy + Zero> WeightedUnionFind<T>
where
    T: std::ops::Neg<Output = T>,
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            par: (0..n).collect::<Vec<usize>>(),
            siz: vec![1; n],
            diff_weight: vec![T::zero(); n],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize, mut w: T) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        w += self.weight(a);
        w -= self.weight(b);
        let a = self.leader(a);
        let b = self.leader(b);
        if a == b {
            return false;
        }
        if self.siz[a] > self.siz[b] {
            self.par[b] = a;
            self.siz[a] += self.siz[b];
            self.diff_weight[b] = w;
        } else {
            self.par[a] = b;
            self.siz[b] += self.siz[a];
            self.diff_weight[a] = -w;
        }
        true
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.par[a] == a {
            return a;
        }
        let leader = self.leader(self.par[a]);
        self.diff_weight[a] = self.diff_weight[a] + self.diff_weight[self.par[a]];
        self.par[a] = leader;
        self.par[a]
    }

    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let a = self.leader(a);
        self.siz[a]
    }

    pub fn diff(&mut self, a: usize, b: usize) -> T {
        assert!(a < self.n);
        assert!(b < self.n);
        assert!(self.same(a, b));
        self.weight(b) - self.weight(a)
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[self.leader(i)].push(i);
        }
        res.into_iter()
            .filter(|f| !f.is_empty())
            .collect::<Vec<_>>()
    }

    fn weight(&mut self, a: usize) -> T {
        self.leader(a);
        self.diff_weight[a]
    }
}
