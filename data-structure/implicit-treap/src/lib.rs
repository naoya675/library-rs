use xorshift_64::XorShift64;

type Link<T, F> = Option<Box<Node<T, F>>>;

#[derive(Debug, Clone)]
struct Node<T, F> {
    val: T,
    prod: T,
    rev_prod: T,
    rev: bool,
    lazy: F,
    size: usize,
    priority: u64,
    l: Link<T, F>,
    r: Link<T, F>,
}

#[derive(Debug, Clone)]
pub struct ImplicitTreap<T, F> {
    root: Link<T, F>,
    rng: XorShift64,
    // Monoids: operation (associativity) + identity element
    op: fn(T, T) -> T,
    e: T,
    mapping: fn(F, T) -> T,
    composition: fn(F, F) -> F,
    id: F,
}

impl<T: Copy, F: Copy> ImplicitTreap<T, F> {
    pub fn new(op: fn(T, T) -> T, e: T, mapping: fn(F, T) -> T, composition: fn(F, F) -> F, id: F) -> Self {
        Self {
            root: None,
            rng: XorShift64::new(XorShift64::seed()),
            op,
            e,
            mapping,
            composition,
            id,
        }
    }

    pub fn build(&mut self, v: &[T]) {
        self.root = None;
        for &x in v {
            let len = self.len();
            self.insert(len, x);
        }
    }

    pub fn len(&self) -> usize {
        Self::size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, k: usize, x: T) {
        assert!(k <= self.len());
        let node = Box::new(Node {
            val: x,
            prod: x,
            rev_prod: x,
            lazy: self.id,
            rev: false,
            size: 1,
            priority: self.rng.next_u64(),
            l: None,
            r: None,
        });
        let root = self.root.take();
        let (l, r) = self.split(root, k);
        let t = self.merge(l, Some(node));
        let t = self.merge(t, r);
        self.root = t;
    }

    pub fn remove(&mut self, k: usize) -> T {
        assert!(k < self.len());
        let root = self.root.take();
        let (l, r) = self.split(root, k);
        let (t, r) = self.split(r, 1);
        let val = t.unwrap().val;
        self.root = self.merge(l, r);
        val
    }

    pub fn get(&mut self, k: usize) -> T {
        assert!(k < self.len());
        let root = self.root.take();
        let (l, r) = self.split(root, k);
        let (t, r) = self.split(r, 1);
        let val = t.as_ref().unwrap().val;
        let t = self.merge(l, t);
        let t = self.merge(t, r);
        self.root = t;
        val
    }

    pub fn set(&mut self, k: usize, x: T) {
        assert!(k < self.len());
        let root = self.root.take();
        let (l, r) = self.split(root, k);
        let (t, r) = self.split(r, 1);
        let mut t = t.unwrap();
        t.val = x;
        self.update(&mut t);
        let t = self.merge(l, Some(t));
        let t = self.merge(t, r);
        self.root = t;
    }

    pub fn prod(&mut self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.len());
        if l == r {
            return self.e;
        }
        let root = self.root.take();
        let (nl, nr) = self.split(root, l);
        let (nt, nr) = self.split(nr, r - l);
        let res = nt.as_ref().map_or(self.e, |n| n.prod);
        let nt = self.merge(nl, nt);
        let nt = self.merge(nt, nr);
        self.root = nt;
        res
    }

    pub fn all_prod(&self) -> T {
        self.root.as_ref().map_or(self.e, |n| n.prod)
    }

    pub fn apply(&mut self, l: usize, r: usize, f: F) {
        assert!(l <= r && r <= self.len());
        if l == r {
            return;
        }
        let root = self.root.take();
        let (nl, nr) = self.split(root, l);
        let (mut nt, nr) = self.split(nr, r - l);
        if let Some(n) = nt.as_mut() {
            self.all_apply(n, f);
        }
        let nt = self.merge(nl, nt);
        self.root = self.merge(nt, nr);
    }

    pub fn reverse(&mut self, l: usize, r: usize) {
        assert!(l <= r && r <= self.len());
        if l == r {
            return;
        }
        let root = self.root.take();
        let (nl, nr) = self.split(root, l);
        let (mut nt, nr) = self.split(nr, r - l);
        if let Some(n) = nt.as_mut() {
            self.toggle(n);
        }
        let nt = self.merge(nl, nt);
        self.root = self.merge(nt, nr);
    }

    pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(l <= self.len());
        assert!(g(self.e));
        let root = self.root.take();
        let (nl, nr) = self.split(root, l);
        let mut acc = self.e;
        let mut add = 0;
        let nr = self.max_right_inner(nr, &g, &mut acc, &mut add);
        self.root = self.merge(nl, nr);
        l + add
    }

    pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(r <= self.len());
        assert!(g(self.e));
        let root = self.root.take();
        let (nl, nr) = self.split(root, r);
        let mut acc = self.e;
        let mut sub = 0;
        let nl = self.min_left_inner(nl, &g, &mut acc, &mut sub);
        self.root = self.merge(nl, nr);
        r - sub
    }

    fn size(node: &Link<T, F>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn get_prod(&self, node: &Link<T, F>) -> T {
        node.as_ref().map_or(self.e, |n| n.prod)
    }

    fn get_rev_prod(&self, node: &Link<T, F>) -> T {
        node.as_ref().map_or(self.e, |n| n.rev_prod)
    }

    fn update(&self, n: &mut Node<T, F>) {
        n.size = 1 + Self::size(&n.l) + Self::size(&n.r);
        let lp = self.get_prod(&n.l);
        let rp = self.get_prod(&n.r);
        n.prod = (self.op)((self.op)(lp, n.val), rp);
        let lrp = self.get_rev_prod(&n.l);
        let rrp = self.get_rev_prod(&n.r);
        n.rev_prod = (self.op)((self.op)(rrp, n.val), lrp);
    }

    fn all_apply(&self, n: &mut Node<T, F>, f: F) {
        n.val = (self.mapping)(f, n.val);
        n.prod = (self.mapping)(f, n.prod);
        n.rev_prod = (self.mapping)(f, n.rev_prod);
        n.lazy = (self.composition)(f, n.lazy);
    }

    fn toggle(&self, n: &mut Node<T, F>) {
        std::mem::swap(&mut n.l, &mut n.r);
        std::mem::swap(&mut n.prod, &mut n.rev_prod);
        n.rev ^= true;
    }

    fn push(&self, n: &mut Node<T, F>) {
        if n.rev {
            if let Some(l) = n.l.as_mut() {
                self.toggle(l);
            }
            if let Some(r) = n.r.as_mut() {
                self.toggle(r);
            }
            n.rev = false;
        }
        let f = n.lazy;
        if let Some(l) = n.l.as_mut() {
            self.all_apply(l, f);
        }
        if let Some(r) = n.r.as_mut() {
            self.all_apply(r, f);
        }
        n.lazy = self.id;
    }

    fn merge(&self, l: Link<T, F>, r: Link<T, F>) -> Link<T, F> {
        let Some(mut l) = l else {
            return r;
        };
        let Some(mut r) = r else {
            return Some(l);
        };
        if l.priority > r.priority {
            self.push(&mut l);
            let lr = l.r.take();
            l.r = self.merge(lr, Some(r));
            self.update(&mut l);
            Some(l)
        } else {
            self.push(&mut r);
            let rl = r.l.take();
            r.l = self.merge(Some(l), rl);
            self.update(&mut r);
            Some(r)
        }
    }

    fn split(&self, t: Link<T, F>, k: usize) -> (Link<T, F>, Link<T, F>) {
        let Some(mut n) = t else {
            return (None, None);
        };
        self.push(&mut n);
        let ls = Self::size(&n.l);
        if k <= ls {
            let nl = n.l.take();
            let (l, r) = self.split(nl, k);
            n.l = r;
            self.update(&mut n);
            (l, Some(n))
        } else {
            let nr = n.r.take();
            let (l, r) = self.split(nr, k - ls - 1);
            n.r = l;
            self.update(&mut n);
            (Some(n), r)
        }
    }

    fn max_right_inner<G>(&self, t: Link<T, F>, g: &G, acc: &mut T, add: &mut usize) -> Link<T, F>
    where
        G: Fn(T) -> bool,
    {
        let Some(mut n) = t else {
            return None;
        };
        self.push(&mut n);
        let with_left = (self.op)(*acc, self.get_prod(&n.l));
        if g(with_left) {
            *acc = with_left;
            *add += Self::size(&n.l);
            let with_node = (self.op)(*acc, n.val);
            if g(with_node) {
                *acc = with_node;
                *add += 1;
                let nr = n.r.take();
                n.r = self.max_right_inner(nr, g, acc, add);
            }
        } else {
            let nl = n.l.take();
            n.l = self.max_right_inner(nl, g, acc, add);
        }
        self.update(&mut n);
        Some(n)
    }

    fn min_left_inner<G>(&self, t: Link<T, F>, g: &G, acc: &mut T, sub: &mut usize) -> Link<T, F>
    where
        G: Fn(T) -> bool,
    {
        let Some(mut n) = t else {
            return None;
        };
        self.push(&mut n);
        let with_right = (self.op)(self.get_prod(&n.r), *acc);
        if g(with_right) {
            *acc = with_right;
            *sub += Self::size(&n.r);
            let with_node = (self.op)(n.val, *acc);
            if g(with_node) {
                *acc = with_node;
                *sub += 1;
                let nl = n.l.take();
                n.l = self.min_left_inner(nl, g, acc, sub);
            }
        } else {
            let nr = n.r.take();
            n.r = self.min_left_inner(nr, g, acc, sub);
        }
        self.update(&mut n);
        Some(n)
    }
}
