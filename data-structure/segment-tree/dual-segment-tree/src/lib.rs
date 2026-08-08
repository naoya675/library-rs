#[derive(Debug, Clone)]
pub struct DualSegmentTree<T, F> {
    n: usize,
    size: usize,
    size_log: usize,
    tree: Vec<T>,
    lazy: Vec<F>,
    mapping: fn(F, T) -> T,
    composition: fn(F, F) -> F,
    id: F,
    // monoid (F, composition, id)
}

impl<T: Copy, F: Copy> DualSegmentTree<T, F> {
    pub fn new(n: usize, e: T, mapping: fn(F, T) -> T, composition: fn(F, F) -> F, id: F) -> Self {
        let size = n.next_power_of_two();
        let size_log = size.ilog2() as usize;
        Self {
            n,
            size,
            size_log,
            tree: vec![e; 2 * size],
            lazy: vec![id; 2 * size],
            mapping,
            composition,
            id,
        }
    }

    pub fn from_slice(v: &[T], e: T, mapping: fn(F, T) -> T, composition: fn(F, F) -> F, id: F) -> Self {
        assert!(v.len() > 0);
        let n = v.len();
        let size = n.next_power_of_two();
        let size_log = size.ilog2() as usize;
        let mut st = Self {
            n,
            size,
            size_log,
            tree: vec![e; 2 * size],
            lazy: vec![id; 2 * size],
            mapping,
            composition,
            id,
        };
        for k in 0..n {
            st.tree[k + size] = v[k];
        }
        st
    }

    pub fn set(&mut self, mut k: usize, x: T) {
        assert!(k < self.n);
        k += self.size;
        for i in (1..=self.size_log).rev() {
            self.push(k >> i);
        }
        self.tree[k] = x;
    }

    pub fn get(&mut self, mut k: usize) -> T {
        assert!(k < self.n);
        k += self.size;
        for i in (1..=self.size_log).rev() {
            self.push(k >> i);
        }
        self.tree[k]
    }

    pub fn apply(&mut self, mut l: usize, mut r: usize, f: F) {
        assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.size_log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        while l < r {
            if l & 1 != 0 {
                self.all_apply(l, f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    fn all_apply(&mut self, k: usize, f: F) {
        if k < self.size {
            self.lazy[k] = (self.composition)(f, self.lazy[k]);
        } else {
            self.tree[k] = (self.mapping)(f, self.tree[k]);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(k << 1 | 0, self.lazy[k]);
        self.all_apply(k << 1 | 1, self.lazy[k]);
        self.lazy[k] = self.id;
    }
}
