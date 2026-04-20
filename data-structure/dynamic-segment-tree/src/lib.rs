pub struct DynamicSegmentTree<T> {
    n: usize,
    root: Option<Box<Node<T>>>,
    op: fn(T, T) -> T,
    e: T,
}

struct Node<T> {
    l: Option<Box<Node<T>>>,
    r: Option<Box<Node<T>>>,
    product: T,
}

impl<T: Copy> DynamicSegmentTree<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        Self { n, root: None, op, e }
    }

    pub fn build(&mut self, v: &[T]) {
        assert!(v.len() == self.n);
        self.root = Some(Self::build_inner(0, self.n, v, self.op));
    }

    pub fn set(&mut self, p: usize, x: T) {
        assert!(p < self.n);
        Self::set_inner(&mut self.root, 0, self.n, p, x, self.op, self.e);
    }

    pub fn get(&self, p: usize) -> T {
        assert!(p < self.n);
        Self::get_inner(&self.root, 0, self.n, p, self.e)
    }

    // [l, r)
    pub fn prod(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        Self::prod_inner(&self.root, 0, self.n, l, r, self.op, self.e)
    }

    pub fn all_prod(&self) -> T {
        self.root.as_ref().map_or(self.e, |n| n.product)
    }

    pub fn apply(&mut self, p: usize, x: T) {
        assert!(p < self.n);
        Self::apply_inner(&mut self.root, 0, self.n, p, x, self.op, self.e);
    }

    pub fn max_right<F>(&self, l: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(self.e));
        let mut product = self.e;
        Self::max_right_inner(&self.root, 0, self.n, l, &f, &mut product, self.op, self.n)
    }

    pub fn min_left<F>(&self, r: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(self.e));
        let mut product = self.e;
        Self::min_left_inner(&self.root, 0, self.n, r, &f, &mut product, self.op)
    }

    fn build_inner(nl: usize, nr: usize, v: &[T], op: fn(T, T) -> T) -> Box<Node<T>> {
        if nr - nl == 1 {
            return Box::new(Node {
                l: None,
                r: None,
                product: v[nl],
            });
        }
        let mid = (nl + nr) >> 1;
        let l = Self::build_inner(nl, mid, v, op);
        let r = Self::build_inner(mid, nr, v, op);
        let product = op(l.product, r.product);
        Box::new(Node {
            l: Some(l),
            r: Some(r),
            product,
        })
    }

    fn set_inner(t: &mut Option<Box<Node<T>>>, nl: usize, nr: usize, p: usize, x: T, op: fn(T, T) -> T, e: T) {
        if nr - nl == 1 {
            *t = Some(Box::new(Node { l: None, r: None, product: x }));
            return;
        }
        if t.is_none() {
            *t = Some(Box::new(Node { l: None, r: None, product: e }));
        }
        let t = t.as_mut().unwrap();
        let mid = (nl + nr) >> 1;
        if p < mid {
            Self::set_inner(&mut t.l, nl, mid, p, x, op, e);
        } else {
            Self::set_inner(&mut t.r, mid, nr, p, x, op, e);
        }
        let lp = t.l.as_ref().map_or(e, |n| n.product);
        let rp = t.r.as_ref().map_or(e, |n| n.product);
        t.product = op(lp, rp);
    }

    fn get_inner(t: &Option<Box<Node<T>>>, nl: usize, nr: usize, p: usize, e: T) -> T {
        match t {
            None => e,
            Some(t) => {
                if nr - nl == 1 {
                    return t.product;
                }
                let mid = (nl + nr) >> 1;
                if p < mid {
                    Self::get_inner(&t.l, nl, mid, p, e)
                } else {
                    Self::get_inner(&t.r, mid, nr, p, e)
                }
            }
        }
    }

    fn prod_inner(t: &Option<Box<Node<T>>>, nl: usize, nr: usize, l: usize, r: usize, op: fn(T, T) -> T, e: T) -> T {
        match t {
            None => e,
            Some(t) => {
                if nr <= l || r <= nl {
                    return e;
                }
                if l <= nl && nr <= r {
                    return t.product;
                }
                let mid = (nl + nr) >> 1;
                let lp = Self::prod_inner(&t.l, nl, mid, l, r, op, e);
                let rp = Self::prod_inner(&t.r, mid, nr, l, r, op, e);
                op(lp, rp)
            }
        }
    }

    fn apply_inner(t: &mut Option<Box<Node<T>>>, nl: usize, nr: usize, p: usize, x: T, op: fn(T, T) -> T, e: T) {
        if nr - nl == 1 {
            let cur = t.as_ref().map_or(e, |n| n.product);
            *t = Some(Box::new(Node {
                l: None,
                r: None,
                product: op(cur, x),
            }));
            return;
        }
        if t.is_none() {
            *t = Some(Box::new(Node { l: None, r: None, product: e }));
        }
        let t = t.as_mut().unwrap();
        let mid = (nl + nr) >> 1;
        if p < mid {
            Self::apply_inner(&mut t.l, nl, mid, p, x, op, e);
        } else {
            Self::apply_inner(&mut t.r, mid, nr, p, x, op, e);
        }
        let lp = t.l.as_ref().map_or(e, |n| n.product);
        let rp = t.r.as_ref().map_or(e, |n| n.product);
        t.product = op(lp, rp);
    }

    fn max_right_inner<F>(t: &Option<Box<Node<T>>>, nl: usize, nr: usize, l: usize, f: &F, product: &mut T, op: fn(T, T) -> T, n: usize) -> usize
    where
        F: Fn(T) -> bool,
    {
        match t {
            None => n,
            Some(t) => {
                if nr <= l {
                    return n;
                }
                if l <= nl && f(op(*product, t.product)) {
                    *product = op(*product, t.product);
                    return n;
                }
                if nr - nl == 1 {
                    return nl;
                }
                let mid = (nl + nr) >> 1;
                let res = Self::max_right_inner(&t.l, nl, mid, l, f, product, op, n);
                if res != n {
                    return res;
                }
                Self::max_right_inner(&t.r, mid, nr, l, f, product, op, n)
            }
        }
    }

    fn min_left_inner<F>(t: &Option<Box<Node<T>>>, nl: usize, nr: usize, r: usize, f: &F, product: &mut T, op: fn(T, T) -> T) -> usize
    where
        F: Fn(T) -> bool,
    {
        match t {
            None => 0,
            Some(t) => {
                if r <= nl {
                    return 0;
                }
                if nr <= r && f(op(t.product, *product)) {
                    *product = op(t.product, *product);
                    return 0;
                }
                if nr - nl == 1 {
                    return nr;
                }
                let mid = (nl + nr) >> 1;
                let res = Self::min_left_inner(&t.r, mid, nr, r, f, product, op);
                if res != 0 {
                    return res;
                }
                Self::min_left_inner(&t.l, nl, mid, r, f, product, op)
            }
        }
    }
}

impl<T: Copy + PartialEq> DynamicSegmentTree<T> {
    pub fn reset(&mut self, l: usize, r: usize) {
        assert!(l <= r && r <= self.n);
        Self::reset_inner(&mut self.root, 0, self.n, l, r, self.op, self.e);
    }

    fn reset_inner(t: &mut Option<Box<Node<T>>>, nl: usize, nr: usize, l: usize, r: usize, op: fn(T, T) -> T, e: T) {
        if t.is_none() || nr <= l || r <= nl {
            return;
        }
        if l <= nl && nr <= r {
            *t = None;
            return;
        }
        let mid = (nl + nr) >> 1;
        let node = t.as_mut().unwrap();
        Self::reset_inner(&mut node.l, nl, mid, l, r, op, e);
        Self::reset_inner(&mut node.r, mid, nr, l, r, op, e);
        let lp = node.l.as_ref().map_or(e, |n| n.product);
        let rp = node.r.as_ref().map_or(e, |n| n.product);
        if lp == e && rp == e {
            *t = None;
        } else {
            t.as_mut().unwrap().product = op(lp, rp);
        }
    }
}
