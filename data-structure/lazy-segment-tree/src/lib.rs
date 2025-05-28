//! https://atcoder.github.io/ac-library/production/document_en/lazysegtree.html

mod wrapper;
pub use wrapper::*;

#[derive(Debug, Clone)]
pub struct LazySegmentTree<T, F> {
    tree: Vec<T>,
    lazy: Vec<F>,
    size: usize,
    size_log: usize,
    // Monoids: associativity (operation) + identity element
    op: fn(T, T) -> T,
    e: T,
    mapping: fn(F, T) -> T,
    composition: fn(F, F) -> F,
    id: F,
    n: usize,
}

impl<T: Copy, F: Copy> LazySegmentTree<T, F> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T, mapping: fn(F, T) -> T, composition: fn(F, F) -> F, id: F) -> Self {
        let size = n.next_power_of_two();
        let size_log = (size.ilog2() + 1) as usize;
        Self {
            tree: vec![e; 2 * size],
            lazy: vec![id; 2 * size],
            size,
            size_log,
            op,
            e,
            mapping,
            composition,
            id,
            n,
        }
    }

    pub fn build(&mut self, v: Vec<T>) {
        assert!(v.len() <= self.n);
        for i in 0..v.len() {
            self.set(i, v[i]);
        }
    }

    pub fn set(&mut self, mut k: usize, x: T) {
        assert!(k < self.n);
        k += self.size;
        for i in (1..self.size_log + 1).rev() {
            self.push(k >> i);
        }
        self.tree[k] = x;
        for i in 1..self.size_log + 1 {
            self.update(k >> i);
        }
    }

    pub fn get(&mut self, mut k: usize) -> T {
        assert!(k < self.n);
        k += self.size;
        for i in (1..self.size_log + 1).rev() {
            self.push(k >> i);
        }
        self.tree[k].clone()
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e;
        }
        l += self.size;
        r += self.size;
        for i in (1..self.size_log + 1).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }
        let mut l_res = self.e;
        let mut r_res = self.e;
        while l < r {
            if l & 1 != 0 {
                l_res = (self.op)(l_res, self.tree[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                r_res = (self.op)(self.tree[r], r_res);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(l_res, r_res)
    }

    pub fn all_prod(&mut self) -> T {
        self.tree[1].clone()
    }

    /*
    pub fn apply(&mut self, mut k: usize, f: F) {
        assert!(k < self.n);
        k += self.size;
        for i in (1..self.size_log + 1).rev() {
            self.push(k >> i);
        }
        self.tree[k] = (self.mapping)(f, self.tree[k]);
        for i in 1..self.size_log + 1 {
            self.update(k >> i);
        }
    }
    */

    pub fn apply(&mut self, mut l: usize, mut r: usize, f: F) {
        assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;
        for i in (1..self.size_log + 1).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let l2 = l;
        let r2 = r;
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
        l = l2;
        r = r2;
        for i in 1..self.size_log + 1 {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(g(self.e));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        for i in (1..self.size_log + 1).rev() {
            self.push(l >> i);
        }
        let mut res = self.e;
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g((self.op)(res, self.tree[l])) {
                while l < self.size {
                    self.push(l);
                    l = 2 * l;
                    if g((self.op)(res, self.tree[l])) {
                        res = (self.op)(res, self.tree[l]);
                        l += 1;
                    }
                }
                return l - self.size;
            }
            res = (self.op)(res, self.tree[l]);
            l += 1;
            l & l.wrapping_neg() != l
        } {}
        self.n
    }

    pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(g(self.e));
        if r == 0 {
            return 0;
        }
        r += self.size;
        for i in (1..self.size_log + 1).rev() {
            self.push((r - 1) >> i);
        }
        let mut res = self.e;
        while {
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !g((self.op)(self.tree[r], res)) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    if g((self.op)(self.tree[r], res)) {
                        res = (self.op)(self.tree[r], res);
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            res = (self.op)(self.tree[r], res);
            r & r.wrapping_neg() != r
        } {}
        0
    }

    fn all_apply(&mut self, k: usize, f: F) {
        self.tree[k] = (self.mapping)(f, self.tree[k]);
        if k < self.size {
            self.lazy[k] = (self.composition)(f, self.lazy[k]);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(k << 1 | 0, self.lazy[k]);
        self.all_apply(k << 1 | 1, self.lazy[k]);
        self.lazy[k] = self.id;
    }

    fn update(&mut self, k: usize) {
        self.tree[k] = (self.op)(self.tree[k << 1 | 0], self.tree[k << 1 | 1]);
    }
}
