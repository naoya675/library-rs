pub struct PersistentSegmentTreePool<T> {
    n: usize,
    nodes: Vec<Node<T>>,
    op: fn(T, T) -> T,
    e: T,
}

struct Node<T> {
    l: Option<u32>,
    r: Option<u32>,
    product: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Version {
    root: Option<u32>,
}

impl<T: Copy> PersistentSegmentTreePool<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        Self { n, nodes: Vec::new(), op, e }
    }

    pub fn with_capacity(n: usize, capacity: usize, op: fn(T, T) -> T, e: T) -> Self {
        Self {
            n,
            nodes: Vec::with_capacity(capacity),
            op,
            e,
        }
    }

    pub fn empty(&self) -> Version {
        Version { root: None }
    }

    pub fn build(&mut self, v: &[T]) -> Version {
        assert!(v.len() == self.n);
        Version {
            root: Some(self.build_inner(0, self.n, v)),
        }
    }

    pub fn set(&mut self, ver: Version, p: usize, x: T) -> Version {
        assert!(p < self.n);
        Version {
            root: Some(self.set_inner(ver.root, 0, self.n, p, x)),
        }
    }

    pub fn get(&self, ver: Version, p: usize) -> T {
        assert!(p < self.n);
        self.get_inner(ver.root, 0, self.n, p)
    }

    // [l, r)
    pub fn prod(&self, ver: Version, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        self.prod_inner(ver.root, 0, self.n, l, r)
    }

    pub fn all_prod(&self, ver: Version) -> T {
        ver.root.map_or(self.e, |i| self.nodes[i as usize].product)
    }

    pub fn apply(&mut self, ver: Version, p: usize, x: T) -> Version {
        assert!(p < self.n);
        Version {
            root: Some(self.apply_inner(ver.root, 0, self.n, p, x)),
        }
    }

    pub fn max_right<F>(&self, ver: Version, l: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(self.e));
        let mut product = self.e;
        self.max_right_inner(ver.root, 0, self.n, l, &f, &mut product)
    }

    pub fn min_left<F>(&self, ver: Version, r: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(self.e));
        let mut product = self.e;
        self.min_left_inner(ver.root, 0, self.n, r, &f, &mut product)
    }

    fn alloc(&mut self, l: Option<u32>, r: Option<u32>, product: T) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(Node { l, r, product });
        idx
    }

    fn build_inner(&mut self, nl: usize, nr: usize, v: &[T]) -> u32 {
        if nr - nl == 1 {
            return self.alloc(None, None, v[nl]);
        }
        let mid = (nl + nr) >> 1;
        let l = self.build_inner(nl, mid, v);
        let r = self.build_inner(mid, nr, v);
        let product = (self.op)(self.nodes[l as usize].product, self.nodes[r as usize].product);
        self.alloc(Some(l), Some(r), product)
    }

    fn set_inner(&mut self, t: Option<u32>, nl: usize, nr: usize, p: usize, x: T) -> u32 {
        if nr - nl == 1 {
            return self.alloc(None, None, x);
        }
        let mid = (nl + nr) >> 1;
        let (l, r) = match t {
            None => (None, None),
            Some(idx) => (self.nodes[idx as usize].l, self.nodes[idx as usize].r),
        };
        let l = if p < mid { Some(self.set_inner(l, nl, mid, p, x)) } else { l };
        let r = if p < mid { r } else { Some(self.set_inner(r, mid, nr, p, x)) };
        let lp = l.map_or(self.e, |i| self.nodes[i as usize].product);
        let rp = r.map_or(self.e, |i| self.nodes[i as usize].product);
        self.alloc(l, r, (self.op)(lp, rp))
    }

    fn get_inner(&self, t: Option<u32>, nl: usize, nr: usize, p: usize) -> T {
        let Some(idx) = t else { return self.e };
        let n = &self.nodes[idx as usize];
        if nr - nl == 1 {
            return n.product;
        }
        let mid = (nl + nr) >> 1;
        if p < mid {
            self.get_inner(n.l, nl, mid, p)
        } else {
            self.get_inner(n.r, mid, nr, p)
        }
    }

    fn prod_inner(&self, t: Option<u32>, nl: usize, nr: usize, l: usize, r: usize) -> T {
        let Some(idx) = t else { return self.e };
        if nr <= l || r <= nl {
            return self.e;
        }
        let n = &self.nodes[idx as usize];
        if l <= nl && nr <= r {
            return n.product;
        }
        let mid = (nl + nr) >> 1;
        let lp = self.prod_inner(n.l, nl, mid, l, r);
        let rp = self.prod_inner(n.r, mid, nr, l, r);
        (self.op)(lp, rp)
    }

    fn apply_inner(&mut self, t: Option<u32>, nl: usize, nr: usize, p: usize, x: T) -> u32 {
        if nr - nl == 1 {
            let cur = t.map_or(self.e, |i| self.nodes[i as usize].product);
            return self.alloc(None, None, (self.op)(cur, x));
        }
        let mid = (nl + nr) >> 1;
        let (l, r) = match t {
            None => (None, None),
            Some(idx) => (self.nodes[idx as usize].l, self.nodes[idx as usize].r),
        };
        let l = if p < mid { Some(self.apply_inner(l, nl, mid, p, x)) } else { l };
        let r = if p < mid { r } else { Some(self.apply_inner(r, mid, nr, p, x)) };
        let lp = l.map_or(self.e, |i| self.nodes[i as usize].product);
        let rp = r.map_or(self.e, |i| self.nodes[i as usize].product);
        self.alloc(l, r, (self.op)(lp, rp))
    }

    fn max_right_inner<F>(&self, t: Option<u32>, nl: usize, nr: usize, l: usize, f: &F, product: &mut T) -> usize
    where
        F: Fn(T) -> bool,
    {
        let Some(idx) = t else { return self.n };
        if nr <= l {
            return self.n;
        }
        let n = &self.nodes[idx as usize];
        if l <= nl && f((self.op)(*product, n.product)) {
            *product = (self.op)(*product, n.product);
            return self.n;
        }
        if nr - nl == 1 {
            return nl;
        }
        let mid = (nl + nr) >> 1;
        let res = self.max_right_inner(n.l, nl, mid, l, f, product);
        if res != self.n {
            return res;
        }
        self.max_right_inner(n.r, mid, nr, l, f, product)
    }

    fn min_left_inner<F>(&self, t: Option<u32>, nl: usize, nr: usize, r: usize, f: &F, product: &mut T) -> usize
    where
        F: Fn(T) -> bool,
    {
        let Some(idx) = t else { return 0 };
        if r <= nl {
            return 0;
        }
        let n = &self.nodes[idx as usize];
        if nr <= r && f((self.op)(n.product, *product)) {
            *product = (self.op)(n.product, *product);
            return 0;
        }
        if nr - nl == 1 {
            return nr;
        }
        let mid = (nl + nr) >> 1;
        let res = self.min_left_inner(n.r, mid, nr, r, f, product);
        if res != 0 {
            return res;
        }
        self.min_left_inner(n.l, nl, mid, r, f, product)
    }
}

impl<T: Copy + PartialEq> PersistentSegmentTreePool<T> {
    pub fn reset(&mut self, ver: Version, l: usize, r: usize) -> Version {
        assert!(l <= r && r <= self.n);
        Version {
            root: self.reset_inner(ver.root, 0, self.n, l, r),
        }
    }

    fn reset_inner(&mut self, t: Option<u32>, nl: usize, nr: usize, l: usize, r: usize) -> Option<u32> {
        let Some(idx) = t else { return None };
        if nr <= l || r <= nl {
            return Some(idx);
        }
        if l <= nl && nr <= r {
            return None;
        }
        let mid = (nl + nr) >> 1;
        let new_l = self.reset_inner(self.nodes[idx as usize].l, nl, mid, l, r);
        let new_r = self.reset_inner(self.nodes[idx as usize].r, mid, nr, l, r);
        let lp = new_l.map_or(self.e, |i| self.nodes[i as usize].product);
        let rp = new_r.map_or(self.e, |i| self.nodes[i as usize].product);
        if lp == self.e && rp == self.e {
            None
        } else {
            Some(self.alloc(new_l, new_r, (self.op)(lp, rp)))
        }
    }
}
