use xorshift_64::XorShift64;

#[derive(Debug, Clone)]
struct Node {
    key: usize,
    priority: u64,
    size: usize,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct Treap {
    root: Option<Box<Node>>,
    rng: XorShift64,
}

impl Treap {
    pub fn new() -> Self {
        Self {
            root: None,
            rng: XorShift64::new(XorShift64::seed()),
        }
    }

    pub fn len(&self) -> usize {
        Self::size(&self.root)
    }

    pub fn insert(&mut self, x: usize) {
        let node = Box::new(Node {
            key: x,
            priority: self.rng.next_u64(),
            size: 1,
            l: None,
            r: None,
        });
        let root = self.root.take();
        let (l, r) = Self::split_by_key(root, x);
        let t = Self::merge_inner(l, Some(node));
        let t = Self::merge_inner(t, r);
        self.root = t;
    }

    pub fn remove(&mut self, x: usize) -> bool {
        let root = self.root.take();
        let (l, r) = Self::split_by_key(root, x);
        let (t, r) = Self::split_at_index(r, 1);
        let removed = t.as_ref().is_some_and(|n| n.key == x);
        self.root = if removed {
            Self::merge_inner(l, r)
        } else {
            let t = Self::merge_inner(l, t);
            let t = Self::merge_inner(t, r);
            t
        };
        removed
    }

    pub fn contains(&self, x: usize) -> bool {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            if n.key == x {
                return true;
            }
            cur = if x < n.key { n.l.as_deref() } else { n.r.as_deref() };
        }
        false
    }

    pub fn count(&self, x: usize) -> usize {
        self.upper_bound(x) - self.lower_bound(x)
    }

    pub fn kth(&self, mut k: usize) -> usize {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            let l_size = Self::size(&n.l);
            if k == l_size {
                return n.key;
            } else if k < l_size {
                cur = n.l.as_deref();
            } else {
                k -= l_size + 1;
                cur = n.r.as_deref();
            }
        }
        unreachable!()
    }

    pub fn min(&self) -> Option<usize> {
        let mut cur = self.root.as_deref()?;
        while let Some(l) = cur.l.as_deref() {
            cur = l;
        }
        Some(cur.key)
    }

    pub fn max(&self) -> Option<usize> {
        let mut cur = self.root.as_deref()?;
        while let Some(r) = cur.r.as_deref() {
            cur = r;
        }
        Some(cur.key)
    }

    pub fn lower_bound(&self, x: usize) -> usize {
        let mut cur = self.root.as_deref();
        let mut pos = 0;
        while let Some(n) = cur {
            if n.key < x {
                pos += Self::size(&n.l) + 1;
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        pos
    }

    pub fn upper_bound(&self, x: usize) -> usize {
        let mut cur = self.root.as_deref();
        let mut pos = 0;
        while let Some(n) = cur {
            if n.key <= x {
                pos += Self::size(&n.l) + 1;
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        pos
    }

    pub fn successor(&self, x: usize) -> Option<usize> {
        let mut cur = self.root.as_deref();
        let mut res = None;
        while let Some(n) = cur {
            if n.key >= x {
                res = Some(n.key);
                cur = n.l.as_deref();
            } else {
                cur = n.r.as_deref();
            }
        }
        res
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        let mut cur = self.root.as_deref();
        let mut res = None;
        while let Some(n) = cur {
            if n.key <= x {
                res = Some(n.key);
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        res
    }

    pub fn split_off(&mut self, x: usize) -> Self {
        let root = self.root.take();
        let (l, r) = Self::split_by_key(root, x);
        self.root = l;
        Self {
            root: r,
            rng: XorShift64::new(self.rng.next_u64()),
        }
    }

    pub fn split_off_at(&mut self, k: usize) -> Self {
        let root = self.root.take();
        let (l, r) = Self::split_at_index(root, k);
        self.root = l;
        Self {
            root: r,
            rng: XorShift64::new(self.rng.next_u64()),
        }
    }

    pub fn merge(&mut self, mut other: Self) {
        debug_assert!('check: {
            let Some(lhs) = self.max() else {
                break 'check true;
            };
            let Some(rhs) = other.min() else {
                break 'check true;
            };
            lhs < rhs
        },);
        let l = self.root.take();
        let r = other.root.take();
        self.root = Self::merge_inner(l, r);
    }

    fn size(node: &Option<Box<Node>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn update(n: &mut Node) {
        n.size = 1 + Self::size(&n.l) + Self::size(&n.r);
    }

    // Split by key: l = {k : k < key}, r = {k : k >= key}.
    fn split_by_key(node: Option<Box<Node>>, key: usize) -> (Option<Box<Node>>, Option<Box<Node>>) {
        let Some(mut n) = node else {
            return (None, None);
        };
        if n.key < key {
            let (l, r) = Self::split_by_key(n.r.take(), key);
            n.r = l;
            Self::update(&mut n);
            (Some(n), r)
        } else {
            let (l, r) = Self::split_by_key(n.l.take(), key);
            n.l = r;
            Self::update(&mut n);
            (l, Some(n))
        }
    }

    // Split at index: l = first k elements, r = rest.
    fn split_at_index(node: Option<Box<Node>>, k: usize) -> (Option<Box<Node>>, Option<Box<Node>>) {
        let Some(mut n) = node else {
            return (None, None);
        };
        let l_size = Self::size(&n.l);
        if k > l_size {
            let (l, r) = Self::split_at_index(n.r.take(), k - l_size - 1);
            n.r = l;
            Self::update(&mut n);
            (Some(n), r)
        } else {
            let (l, r) = Self::split_at_index(n.l.take(), k);
            n.l = r;
            Self::update(&mut n);
            (l, Some(n))
        }
    }

    fn merge_inner(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
        let Some(mut l) = l else {
            return r;
        };
        let Some(mut r) = r else {
            return Some(l);
        };
        if l.priority > r.priority {
            l.r = Self::merge_inner(l.r.take(), Some(r));
            Self::update(&mut l);
            Some(l)
        } else {
            r.l = Self::merge_inner(Some(l), r.l.take());
            Self::update(&mut r);
            Some(r)
        }
    }
}
