#[derive(Debug, Clone)]
pub struct UnionFindWithRollbackAbstract<T> {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
    val: Vec<T>,
    history: Vec<Option<usize>>,
    op: fn(T, T) -> T,
    e: T,
    inv: fn(T) -> T,
    // abelian group (T, op, e, inv)
}

impl<T: Copy> UnionFindWithRollbackAbstract<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self {
        Self {
            n,
            par: (0..n).collect(),
            siz: vec![1; n],
            val: vec![e; n],
            history: vec![],
            op,
            e,
            inv,
        }
    }

    pub fn from_slice(v: &[T], op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self {
        let n = v.len();
        Self {
            n,
            par: (0..n).collect(),
            siz: vec![1; n],
            val: v.to_vec(),
            history: vec![],
            op,
            e,
            inv,
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
        self.val[x] = (self.op)(self.val[x], self.val[y]);
        self.par[y] = x;
        self.history.push(Some(y));
        x
    }

    pub fn apply(&mut self, x: usize, val: T) {
        assert!(x < self.n);
        let mut x = x;
        loop {
            self.val[x] = (self.op)(self.val[x], val);
            if self.par[x] == x {
                break;
            }
            x = self.par[x];
        }
    }

    pub fn prod(&self, x: usize) -> T {
        assert!(x < self.n);
        self.val[self.leader(x)]
    }

    pub fn snapshot(&self) -> usize {
        self.history.len()
    }

    pub fn rollback(&mut self) {
        if let Some(Some(y)) = self.history.pop() {
            let x = self.par[y];
            self.siz[x] -= self.siz[y];
            self.val[x] = (self.op)(self.val[x], (self.inv)(self.val[y]));
            self.par[y] = y;
        }
    }

    pub fn rollback_to(&mut self, snap: usize) {
        while self.history.len() > snap {
            self.rollback();
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
