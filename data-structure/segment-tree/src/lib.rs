// reference: https://atcoder.github.io/ac-library/production/document_en/segtree.html

mod wrapper;
pub use wrapper::*;

#[derive(Debug, Clone)]
pub struct SegmentTree<T> {
    tree: Vec<T>,
    size: usize,
    size_log: usize,
    // Monoids: operation (associativity) + identity element
    op: fn(T, T) -> T,
    e: T,
    n: usize,
}

impl<T: Copy> SegmentTree<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        let size = n.next_power_of_two();
        let size_log = (size.ilog2() + 1) as usize;
        Self {
            tree: vec![e; 2 * size],
            size,
            size_log,
            op,
            e,
            n,
        }
    }

    pub fn build(&mut self, vec: Vec<T>) {
        assert!(vec.len() == self.n);
        for k in 0..self.n {
            self.tree[k + self.size] = vec[k];
        }
        for k in (0..self.size).rev() {
            self.update(k);
        }
    }

    pub fn set(&mut self, mut k: usize, x: T) {
        assert!(k < self.n);
        k += self.size;
        self.tree[k] = x;
        for i in 1..self.size_log + 1 {
            self.update(k >> i);
        }
        /*
        while k > 0 {
            k >>= 1;
            self.update(k);
        }
        */
    }

    pub fn get(&mut self, mut k: usize) -> T {
        assert!(k < self.n);
        k += self.size;
        self.tree[k].clone()
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e;
        }
        l += self.size;
        r += self.size;
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
    pub fn apply(&mut self, mut k: usize, x: T) {
        assert!(k < self.n);
        k += self.size;
        self.tree[k] = (self.op)(self.tree[k], x);
        while k > 0 {
            k >>= 1;
            self.update(k);
        }
    }
    */

    pub fn apply(&mut self, k: usize, x: T) {
        self.apply_with(k, x, self.op)
    }

    pub fn apply_with<F>(&mut self, mut k: usize, x: T, f: F)
    where
        F: Fn(T, T) -> T,
    {
        assert!(k < self.n);
        k += self.size;
        self.tree[k] = f(self.tree[k], x);
        while k > 0 {
            k >>= 1;
            self.update(k);
        }
    }

    pub fn max_right<F>(&mut self, mut l: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(self.e));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut res = self.e;
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f((self.op)(res, self.tree[l])) {
                while l < self.size {
                    l = 2 * l;
                    if f((self.op)(res, self.tree[l])) {
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

    pub fn min_left<F>(&mut self, mut r: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(self.e));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut res = self.e;
        while {
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !f((self.op)(self.tree[r], res)) {
                while r < self.size {
                    r = 2 * r + 1;
                    if f((self.op)(self.tree[r], res)) {
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

    fn update(&mut self, k: usize) {
        self.tree[k] = (self.op)(self.tree[k << 1 | 0], self.tree[k << 1 | 1]);
    }
}
