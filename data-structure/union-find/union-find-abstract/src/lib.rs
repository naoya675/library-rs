#[derive(Debug, Clone)]
pub struct UnionFindAbstract<T> {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
    val: Vec<T>,
    op: fn(T, T) -> T,
    e: T,
    // commutative monoid (T, op, e)
}

impl<T: Copy> UnionFindAbstract<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        Self {
            n,
            par: (0..n).collect(),
            siz: vec![1; n],
            val: vec![e; n],
            op,
            e,
        }
    }

    pub fn from_slice(v: &[T], op: fn(T, T) -> T, e: T) -> Self {
        let n = v.len();
        Self {
            n,
            par: (0..n).collect(),
            siz: vec![1; n],
            val: v.to_vec(),
            op,
            e,
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
        self.val[x] = (self.op)(self.val[x], self.val[y]);
        self.par[y] = x;
        x
    }

    pub fn apply(&mut self, x: usize, val: T) {
        assert!(x < self.n);
        let x = self.leader(x);
        self.val[x] = (self.op)(self.val[x], val);
    }

    pub fn prod(&mut self, x: usize) -> T {
        assert!(x < self.n);
        let x = self.leader(x);
        self.val[x]
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
        res.into_iter().filter(|f| !f.is_empty()).collect()
    }
}
