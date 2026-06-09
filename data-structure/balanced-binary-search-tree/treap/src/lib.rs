use xorshift_64::XorShift64;

#[derive(Debug, Clone)]
struct Node<T: Ord> {
    key: T,
    priority: u64,
    size: usize,
    l: Option<Box<Node<T>>>,
    r: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct Treap<T: Ord> {
    root: Option<Box<Node<T>>>,
    rng: XorShift64,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            rng: XorShift64::default(),
        }
    }

    pub fn len(&self) -> usize {
        Self::size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, x: T) -> bool {
        let node = Box::new(Node {
            key: x,
            priority: self.rng.next_u64(),
            size: 1,
            l: None,
            r: None,
        });
        Self::insert_inner(&mut self.root, node)
    }

    pub fn remove(&mut self, x: &T) -> bool {
        Self::remove_inner(&mut self.root, x)
    }

    pub fn contains(&self, x: &T) -> bool {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            if x == &n.key {
                return true;
            } else if x < &n.key {
                cur = n.l.as_deref();
            } else {
                cur = n.r.as_deref();
            }
        }
        false
    }

    pub fn kth(&self, mut k: usize) -> &T {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            let l_size = Self::size(&n.l);
            if k == l_size {
                return &n.key;
            } else if k < l_size {
                cur = n.l.as_deref();
            } else {
                k -= l_size + 1;
                cur = n.r.as_deref();
            }
        }
        unreachable!()
    }

    pub fn min(&self) -> Option<&T> {
        let mut cur = self.root.as_deref()?;
        while let Some(l) = cur.l.as_deref() {
            cur = l;
        }
        Some(&cur.key)
    }

    pub fn max(&self) -> Option<&T> {
        let mut cur = self.root.as_deref()?;
        while let Some(r) = cur.r.as_deref() {
            cur = r;
        }
        Some(&cur.key)
    }

    pub fn pop_min(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            _ => {
                let (t, r) = Self::split_inner_index(self.root.take(), 1);
                self.root = r;
                t.map(|n| n.key)
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            len => {
                let (l, t) = Self::split_inner_index(self.root.take(), len - 1);
                self.root = l;
                t.map(|n| n.key)
            }
        }
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    pub fn lower_bound(&self, x: &T) -> usize {
        let mut cur = self.root.as_deref();
        let mut pos = 0;
        while let Some(n) = cur {
            if &n.key < x {
                pos += Self::size(&n.l) + 1;
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        pos
    }

    pub fn upper_bound(&self, x: &T) -> usize {
        let mut cur = self.root.as_deref();
        let mut pos = 0;
        while let Some(n) = cur {
            if &n.key <= x {
                pos += Self::size(&n.l) + 1;
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        pos
    }

    pub fn successor(&self, x: &T) -> Option<&T> {
        let mut cur = self.root.as_deref();
        let mut res = None;
        while let Some(n) = cur {
            if &n.key >= x {
                res = Some(&n.key);
                cur = n.l.as_deref();
            } else {
                cur = n.r.as_deref();
            }
        }
        res
    }

    pub fn predecessor(&self, x: &T) -> Option<&T> {
        let mut cur = self.root.as_deref();
        let mut res = None;
        while let Some(n) = cur {
            if &n.key <= x {
                res = Some(&n.key);
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        res
    }

    pub fn split_off(&mut self, x: &T) -> Self {
        let root = self.root.take();
        let (l, r) = Self::split_inner(root, x);
        self.root = l;
        Self {
            root: r,
            rng: XorShift64::new(self.rng.next_u64()),
        }
    }

    pub fn split_off_at(&mut self, k: usize) -> Self {
        let root = self.root.take();
        let (l, r) = Self::split_inner_index(root, k);
        self.root = l;
        Self {
            root: r,
            rng: XorShift64::new(self.rng.next_u64()),
        }
    }

    pub fn merge(&mut self, mut rhs: Self) {
        debug_assert!(match (self.max(), rhs.min()) {
            (Some(l), Some(r)) => l < r,
            _ => true,
        });
        let l = self.root.take();
        Self::merge_inner(&mut self.root, l, rhs.root.take());
    }

    pub fn union(&mut self, rhs: Self) {
        self.root = Self::unite(self.root.take(), rhs.root);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        let mut next = vec![];
        let mut back = vec![];
        let mut cur = &self.root;
        while let Some(n) = cur {
            next.push(n.as_ref());
            cur = &n.l;
        }
        let mut cur = &self.root;
        while let Some(n) = cur {
            back.push(n.as_ref());
            cur = &n.r;
        }
        Iter {
            next,
            back,
            remaining: self.len(),
        }
    }

    fn size(node: &Option<Box<Node<T>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn update(n: &mut Node<T>) {
        n.size = 1 + Self::size(&n.l) + Self::size(&n.r);
    }

    fn rotate_right(t: &mut Option<Box<Node<T>>>) {
        let mut node = t.take().unwrap();
        let mut l = node.l.take().unwrap();
        node.l = l.r.take();
        Self::update(&mut node);
        l.r = Some(node);
        Self::update(&mut l);
        *t = Some(l);
    }

    fn rotate_left(t: &mut Option<Box<Node<T>>>) {
        let mut node = t.take().unwrap();
        let mut r = node.r.take().unwrap();
        node.r = r.l.take();
        Self::update(&mut node);
        r.l = Some(node);
        Self::update(&mut r);
        *t = Some(r);
    }

    fn insert_inner(t: &mut Option<Box<Node<T>>>, it: Box<Node<T>>) -> bool {
        let Some(node) = t.as_deref() else {
            *t = Some(it);
            return true;
        };
        match it.key.cmp(&node.key) {
            std::cmp::Ordering::Equal => false,
            // std::cmp::Ordering::Equal => {
            //     let node = t.as_mut().unwrap();
            //     Self::insert_inner(&mut node.r, it);
            //     Self::update(node);
            //     if node.r.as_ref().unwrap().priority > node.priority {
            //         Self::rotate_left(t);
            //     }
            //     true
            // }
            std::cmp::Ordering::Less => {
                let node = t.as_mut().unwrap();
                let inserted = Self::insert_inner(&mut node.l, it);
                if inserted {
                    Self::update(node);
                    if node.l.as_ref().unwrap().priority > node.priority {
                        Self::rotate_right(t);
                    }
                }
                inserted
            }
            std::cmp::Ordering::Greater => {
                let node = t.as_mut().unwrap();
                let inserted = Self::insert_inner(&mut node.r, it);
                if inserted {
                    Self::update(node);
                    if node.r.as_ref().unwrap().priority > node.priority {
                        Self::rotate_left(t);
                    }
                }
                inserted
            }
        }
    }

    fn remove_inner(t: &mut Option<Box<Node<T>>>, x: &T) -> bool {
        let Some(node) = t.as_deref() else {
            return false;
        };
        match x.cmp(&node.key) {
            std::cmp::Ordering::Equal => {
                let node = t.take().unwrap();
                Self::merge_inner(t, node.l, node.r);
                true
            }
            std::cmp::Ordering::Less => {
                let node = t.as_mut().unwrap();
                let removed = Self::remove_inner(&mut node.l, x);
                if removed {
                    Self::update(node);
                }
                removed
            }
            std::cmp::Ordering::Greater => {
                let node = t.as_mut().unwrap();
                let removed = Self::remove_inner(&mut node.r, x);
                if removed {
                    Self::update(node);
                }
                removed
            }
        }
    }

    fn split_inner(node: Option<Box<Node<T>>>, key: &T) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        let Some(mut n) = node else {
            return (None, None);
        };
        if &n.key < key {
            let (l, r) = Self::split_inner(n.r.take(), key);
            n.r = l;
            Self::update(&mut n);
            (Some(n), r)
        } else {
            let (l, r) = Self::split_inner(n.l.take(), key);
            n.l = r;
            Self::update(&mut n);
            (l, Some(n))
        }
    }

    fn split_inner_index(node: Option<Box<Node<T>>>, k: usize) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        let Some(mut n) = node else {
            return (None, None);
        };
        let l_size = Self::size(&n.l);
        if k > l_size {
            let (l, r) = Self::split_inner_index(n.r.take(), k - l_size - 1);
            n.r = l;
            Self::update(&mut n);
            (Some(n), r)
        } else {
            let (l, r) = Self::split_inner_index(n.l.take(), k);
            n.l = r;
            Self::update(&mut n);
            (l, Some(n))
        }
    }

    fn merge_inner(t: &mut Option<Box<Node<T>>>, l: Option<Box<Node<T>>>, r: Option<Box<Node<T>>>) {
        let Some(mut l) = l else {
            *t = r;
            return;
        };
        let Some(mut r) = r else {
            *t = Some(l);
            return;
        };
        if l.priority > r.priority {
            let lr = l.r.take();
            Self::merge_inner(&mut l.r, lr, Some(r));
            Self::update(&mut l);
            *t = Some(l);
        } else {
            let rl = r.l.take();
            Self::merge_inner(&mut r.l, Some(l), rl);
            Self::update(&mut r);
            *t = Some(r);
        }
    }

    fn unite(l: Option<Box<Node<T>>>, r: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let Some(mut l) = l else {
            return r;
        };
        let Some(mut r) = r else {
            return Some(l);
        };
        if l.priority < r.priority {
            std::mem::swap(&mut l, &mut r);
        }
        let (lt, rt) = Self::split_inner(Some(r), &l.key);
        l.l = Self::unite(l.l.take(), lt);
        l.r = Self::unite(l.r.take(), rt);
        Self::update(&mut l);
        Some(l)
    }
}

pub struct Iter<'a, T: Ord> {
    next: Vec<&'a Node<T>>,
    back: Vec<&'a Node<T>>,
    remaining: usize,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let node = self.next.pop()?;
        self.remaining -= 1;
        let mut cur = &node.r;
        while let Some(n) = cur {
            self.next.push(n);
            cur = &n.l;
        }
        Some(&node.key)
    }
}

impl<'a, T: Ord> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let node = self.back.pop()?;
        self.remaining -= 1;
        let mut cur = &node.l;
        while let Some(n) = cur {
            self.back.push(n);
            cur = &n.r;
        }
        Some(&node.key)
    }
}

impl<'a, T: Ord> IntoIterator for &'a Treap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Ord> FromIterator<T> for Treap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut treap = Treap::new();
        for x in iter {
            treap.insert(x);
        }
        treap
    }
}
