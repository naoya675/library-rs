#[derive(Debug, Clone)]
pub struct SegmentTree<T> {
    tree: Vec<T>,
    size: usize,
    size_log: usize,
    op: fn(T, T) -> T, // evaluation funciton
    e: T,              // identity element
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

    pub fn build(&mut self, v: Vec<T>) {
        assert!(v.len() <= self.n);
        for i in 0..v.len() {
            self.set(i, v[i]);
        }
    }

    pub fn set(&mut self, mut k: usize, x: T) {
        assert!(k < self.n);
        k += self.size;
        self.tree[k] = x;
        for i in 1..self.size_log + 1 {
            self.update(k >> i);
        }
        // while k > 0 {
        //     k >>= 1;
        //     self.update(k);
        // }
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

    pub fn apply(&mut self, mut k: usize, x: T) {
        assert!(k < self.n);
        k += self.size;
        self.tree[k] = (self.op)(self.tree[k], x);
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

pub struct RangeMinimumQuery;

impl RangeMinimumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |a, b| std::cmp::min(a, b), i64::MAX)
    }
}

pub struct RangeMaximumQuery;

impl RangeMaximumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |a, b| std::cmp::max(a, b), i64::MIN)
    }
}

pub struct RangeSumQuery;

impl RangeSumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |a, b| a + b, 0)
    }
}

pub struct ParenthesisCheckQuery;

impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(
            n,
            |a, b| {
                (
                    a.0 + std::cmp::max(b.0 - a.1, 0),
                    std::cmp::max(a.1 - b.0, 0) + b.1,
                )
            },
            (0, 0),
        )
    }
}

// '(' == (0, 1)
// ')' == (1, 0)

/*
pub struct ParenthesisCheckQuery;

impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(
            n,
            |a, b| (std::cmp::min(a.0, a.1 + b.0), a.1 + b.1),
            (0, 0),
        )
    }
}

// '(' == (0, 1)
// ')' == (-1, -1)
// reference: https://atcoder.jp/contests/abc223/editorial/2774
*/
