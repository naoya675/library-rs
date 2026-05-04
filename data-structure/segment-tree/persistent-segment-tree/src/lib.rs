use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PersistentSegmentTree<T> {
    n: usize,
    root: Option<Rc<Node<T>>>,
    op: fn(T, T) -> T,
    e: T,
}

#[derive(Debug, Clone)]
struct Node<T> {
    l: Option<Rc<Node<T>>>,
    r: Option<Rc<Node<T>>>,
    product: T,
}

impl<T: Copy> PersistentSegmentTree<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        Self { n, root: None, op, e }
    }

    pub fn build(&self, v: &[T]) -> Self {
        assert!(v.len() == self.n);
        Self {
            n: self.n,
            root: Some(self.build_inner(0, self.n, v)),
            op: self.op,
            e: self.e,
        }
    }

    pub fn set(&self, p: usize, x: T) -> Self {
        assert!(p < self.n);
        let root = Some(self.set_inner(self.root.as_ref(), 0, self.n, p, x));
        Self {
            n: self.n,
            root,
            op: self.op,
            e: self.e,
        }
    }

    pub fn get(&self, p: usize) -> T {
        assert!(p < self.n);
        self.get_inner(self.root.as_ref(), 0, self.n, p)
    }

    // [l, r)
    pub fn prod(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        self.prod_inner(self.root.as_ref(), 0, self.n, l, r)
    }

    pub fn all_prod(&self) -> T {
        self.root.as_ref().map_or(self.e, |n| n.product)
    }

    pub fn apply(&self, p: usize, x: T) -> Self {
        assert!(p < self.n);
        let root = Some(self.apply_inner(self.root.as_ref(), 0, self.n, p, x));
        Self {
            n: self.n,
            root,
            op: self.op,
            e: self.e,
        }
    }

    pub fn max_right<F>(&self, l: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(self.e));
        let mut product = self.e;
        self.max_right_inner(self.root.as_ref(), 0, self.n, l, &f, &mut product)
    }

    pub fn min_left<F>(&self, r: usize, f: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(self.e));
        let mut product = self.e;
        self.min_left_inner(self.root.as_ref(), 0, self.n, r, &f, &mut product)
    }

    fn build_inner(&self, nl: usize, nr: usize, v: &[T]) -> Rc<Node<T>> {
        if nr - nl == 1 {
            return Rc::new(Node {
                l: None,
                r: None,
                product: v[nl],
            });
        }
        let mid = (nl + nr) >> 1;
        let l = self.build_inner(nl, mid, v);
        let r = self.build_inner(mid, nr, v);
        let product = (self.op)(l.product, r.product);
        Rc::new(Node {
            l: Some(l),
            r: Some(r),
            product,
        })
    }

    fn set_inner(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, p: usize, x: T) -> Rc<Node<T>> {
        if nr - nl == 1 {
            return Rc::new(Node { l: None, r: None, product: x });
        }
        let mid = (nl + nr) >> 1;
        let (l, r) = match t {
            None => (None, None),
            Some(t) => (t.l.clone(), t.r.clone()),
        };
        let l = if p < mid { Some(self.set_inner(l.as_ref(), nl, mid, p, x)) } else { l };
        let r = if p < mid { r } else { Some(self.set_inner(r.as_ref(), mid, nr, p, x)) };
        let lp = l.as_ref().map_or(self.e, |n| n.product);
        let rp = r.as_ref().map_or(self.e, |n| n.product);
        Rc::new(Node {
            l,
            r,
            product: (self.op)(lp, rp),
        })
    }

    fn get_inner(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, p: usize) -> T {
        let Some(t) = t else { return self.e };
        if nr - nl == 1 {
            return t.product;
        }
        let mid = (nl + nr) >> 1;
        if p < mid {
            self.get_inner(t.l.as_ref(), nl, mid, p)
        } else {
            self.get_inner(t.r.as_ref(), mid, nr, p)
        }
    }

    fn prod_inner(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, l: usize, r: usize) -> T {
        let Some(t) = t else { return self.e };
        if nr <= l || r <= nl {
            return self.e;
        }
        if l <= nl && nr <= r {
            return t.product;
        }
        let mid = (nl + nr) >> 1;
        let lp = self.prod_inner(t.l.as_ref(), nl, mid, l, r);
        let rp = self.prod_inner(t.r.as_ref(), mid, nr, l, r);
        (self.op)(lp, rp)
    }

    fn apply_inner(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, p: usize, x: T) -> Rc<Node<T>> {
        if nr - nl == 1 {
            let cur = t.map_or(self.e, |n| n.product);
            return Rc::new(Node {
                l: None,
                r: None,
                product: (self.op)(cur, x),
            });
        }
        let mid = (nl + nr) >> 1;
        let (l, r) = match t {
            None => (None, None),
            Some(t) => (t.l.clone(), t.r.clone()),
        };
        let l = if p < mid { Some(self.apply_inner(l.as_ref(), nl, mid, p, x)) } else { l };
        let r = if p < mid { r } else { Some(self.apply_inner(r.as_ref(), mid, nr, p, x)) };
        let lp = l.as_ref().map_or(self.e, |n| n.product);
        let rp = r.as_ref().map_or(self.e, |n| n.product);
        Rc::new(Node {
            l,
            r,
            product: (self.op)(lp, rp),
        })
    }

    fn max_right_inner<F>(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, l: usize, f: &F, product: &mut T) -> usize
    where
        F: Fn(T) -> bool,
    {
        let Some(t) = t else { return self.n };
        if nr <= l {
            return self.n;
        }
        if l <= nl && f((self.op)(*product, t.product)) {
            *product = (self.op)(*product, t.product);
            return self.n;
        }
        if nr - nl == 1 {
            return nl;
        }
        let mid = (nl + nr) >> 1;
        let res = self.max_right_inner(t.l.as_ref(), nl, mid, l, f, product);
        if res != self.n {
            return res;
        }
        self.max_right_inner(t.r.as_ref(), mid, nr, l, f, product)
    }

    fn min_left_inner<F>(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, r: usize, f: &F, product: &mut T) -> usize
    where
        F: Fn(T) -> bool,
    {
        let Some(t) = t else { return 0 };
        if r <= nl {
            return 0;
        }
        if nr <= r && f((self.op)(t.product, *product)) {
            *product = (self.op)(t.product, *product);
            return 0;
        }
        if nr - nl == 1 {
            return nr;
        }
        let mid = (nl + nr) >> 1;
        let res = self.min_left_inner(t.r.as_ref(), mid, nr, r, f, product);
        if res != 0 {
            return res;
        }
        self.min_left_inner(t.l.as_ref(), nl, mid, r, f, product)
    }
}

impl<T: Copy + PartialEq> PersistentSegmentTree<T> {
    pub fn reset(&self, l: usize, r: usize) -> Self {
        assert!(l <= r && r <= self.n);
        let root = self.reset_inner(self.root.as_ref(), 0, self.n, l, r);
        Self {
            n: self.n,
            root,
            op: self.op,
            e: self.e,
        }
    }

    fn reset_inner(&self, t: Option<&Rc<Node<T>>>, nl: usize, nr: usize, l: usize, r: usize) -> Option<Rc<Node<T>>> {
        let Some(t) = t else { return None };
        if nr <= l || r <= nl {
            return Some(t.clone());
        }
        if l <= nl && nr <= r {
            return None;
        }
        let mid = (nl + nr) >> 1;
        let new_l = self.reset_inner(t.l.as_ref(), nl, mid, l, r);
        let new_r = self.reset_inner(t.r.as_ref(), mid, nr, l, r);
        let lp = new_l.as_ref().map_or(self.e, |n| n.product);
        let rp = new_r.as_ref().map_or(self.e, |n| n.product);
        if lp == self.e && rp == self.e {
            None
        } else {
            Some(Rc::new(Node {
                l: new_l,
                r: new_r,
                product: (self.op)(lp, rp),
            }))
        }
    }
}
