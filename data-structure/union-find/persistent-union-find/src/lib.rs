use persistent_array::PersistentArray;

#[derive(Debug, Clone)]
pub struct PersistentUnionFind {
    n: usize,
    par: PersistentArray<usize>,
    siz: PersistentArray<usize>,
}

impl PersistentUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            par: PersistentArray::from((0..n).collect::<Vec<usize>>()),
            siz: PersistentArray::from(vec![1; n]),
        }
    }

    pub fn merge(&self, x: usize, y: usize) -> Self {
        assert!(x < self.n);
        assert!(y < self.n);
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return self.clone();
        }
        let sx = self.siz.get(x);
        let sy = self.siz.get(y);
        if sx < sy {
            std::mem::swap(&mut x, &mut y);
        }
        let par = self.par.set(y, x);
        let siz = self.siz.set(x, sx + sy);
        Self { n: self.n, par, siz }
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        self.leader(x) == self.leader(y)
    }

    pub fn leader(&self, x: usize) -> usize {
        assert!(x < self.n);
        let p = self.par.get(x);
        if p == x { x } else { self.leader(p) }
    }

    pub fn size(&self, x: usize) -> usize {
        assert!(x < self.n);
        let r = self.leader(x);
        self.siz.get(r)
    }
}
