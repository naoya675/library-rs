#[derive(Debug, Clone)]
pub struct UnionFindWithPotential<T> {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
    diff_potential: Vec<T>,
    // Monoids: associative (operation) + identity element
    op: fn(T, T) -> T,
    e: T,
    // Group: associative (operation) + identity element + inverse element
    inv: fn(T) -> T,
}

impl<T: Copy + PartialEq + Eq + Default> UnionFindWithPotential<T>
where
    T: std::ops::Neg<Output = T>,
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
{
    pub fn new_default(n: usize) -> Self {
        fn op<T>(a: T, b: T) -> T
        where
            T: std::ops::Add<T, Output = T>,
            T: std::ops::AddAssign,
        {
            a + b
        }

        fn neg<T>(a: T) -> T
        where
            T: std::ops::Neg<Output = T>,
        {
            a.neg()
        }

        Self::new(n, op, T::default(), neg)
    }
}

impl<T: Copy + PartialEq + Eq> UnionFindWithPotential<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self {
        Self {
            n,
            par: (0..n).collect::<Vec<usize>>(),
            siz: vec![1; n],
            diff_potential: vec![e; n],
            op,
            e,
            inv,
        }
    }

    pub fn merge(&mut self, x: usize, y: usize, mut w: T) -> Option<usize> {
        assert!(x < self.n);
        assert!(y < self.n);
        w = (self.op)(self.potential(x), (self.inv)((self.op)(self.potential(y), w)));
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return if w == self.e { Some(x) } else { None };
        }
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
            w = (self.inv)(w);
        }
        self.siz[x] += self.siz[y];
        self.par[y] = x;
        self.diff_potential[y] = w;
        Some(x)
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
        self.diff_potential[x] = (self.op)(self.diff_potential[self.par[x]], self.diff_potential[x]);
        self.par[x] = leader;
        self.par[x]
    }

    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x = self.leader(x);
        self.siz[x]
    }

    pub fn diff(&mut self, x: usize, y: usize) -> T {
        assert!(x < self.n);
        assert!(y < self.n);
        assert!(self.same(x, y));
        (self.op)((self.inv)(self.potential(y)), self.potential(x))
    }

    fn potential(&mut self, x: usize) -> T {
        self.leader(x);
        self.diff_potential[x]
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[self.leader(i)].push(i);
        }
        res.into_iter().filter(|f| !f.is_empty()).collect::<Vec<_>>()
    }
}
