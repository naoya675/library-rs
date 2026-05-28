use xorshift_64::XorShift64;

#[derive(Debug, Clone)]
struct Node<T: Ord, S> {
    key: T,
    value: S,
    priority: u64,
    size: usize,
    l: Option<Box<Node<T, S>>>,
    r: Option<Box<Node<T, S>>>,
}

#[derive(Debug, Clone)]
pub struct TreapMap<T: Ord, S> {
    root: Option<Box<Node<T, S>>>,
    rng: XorShift64,
}

impl<T: Ord, S> TreapMap<T, S> {
    pub fn new() -> Self {
        Self {
            root: None,
            rng: XorShift64::new(XorShift64::seed()),
        }
    }

    pub fn len(&self) -> usize {
        Self::size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, key: T, value: S) -> Option<S> {
        let node = Box::new(Node {
            key,
            value,
            priority: self.rng.next_u64(),
            size: 1,
            l: None,
            r: None,
        });
        Self::insert_inner(&mut self.root, node)
    }

    pub fn remove(&mut self, key: &T) -> Option<S> {
        Self::remove_inner(&mut self.root, key)
    }

    pub fn contains_key(&self, key: &T) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &T) -> Option<&S> {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            if key == &n.key {
                return Some(&n.value);
            } else if key < &n.key {
                cur = n.l.as_deref();
            } else {
                cur = n.r.as_deref();
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut S> {
        let mut cur = self.root.as_deref_mut();
        while let Some(n) = cur {
            if key == &n.key {
                return Some(&mut n.value);
            } else if key < &n.key {
                cur = n.l.as_deref_mut();
            } else {
                cur = n.r.as_deref_mut();
            }
        }
        None
    }

    pub fn kth(&self, mut k: usize) -> (&T, &S) {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            let l_size = Self::size(&n.l);
            if k == l_size {
                return (&n.key, &n.value);
            } else if k < l_size {
                cur = n.l.as_deref();
            } else {
                k -= l_size + 1;
                cur = n.r.as_deref();
            }
        }
        unreachable!()
    }

    pub fn min(&self) -> Option<(&T, &S)> {
        let mut cur = self.root.as_deref()?;
        while let Some(l) = cur.l.as_deref() {
            cur = l;
        }
        Some((&cur.key, &cur.value))
    }

    pub fn max(&self) -> Option<(&T, &S)> {
        let mut cur = self.root.as_deref()?;
        while let Some(r) = cur.r.as_deref() {
            cur = r;
        }
        Some((&cur.key, &cur.value))
    }

    pub fn pop_min(&mut self) -> Option<(T, S)> {
        match self.len() {
            0 => None,
            _ => {
                let (t, r) = Self::split_inner_index(self.root.take(), 1);
                self.root = r;
                t.map(|n| (n.key, n.value))
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<(T, S)> {
        match self.len() {
            0 => None,
            len => {
                let (l, t) = Self::split_inner_index(self.root.take(), len - 1);
                self.root = l;
                t.map(|n| (n.key, n.value))
            }
        }
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    pub fn lower_bound(&self, key: &T) -> usize {
        let mut cur = self.root.as_deref();
        let mut pos = 0;
        while let Some(n) = cur {
            if &n.key < key {
                pos += Self::size(&n.l) + 1;
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        pos
    }

    pub fn upper_bound(&self, key: &T) -> usize {
        let mut cur = self.root.as_deref();
        let mut pos = 0;
        while let Some(n) = cur {
            if &n.key <= key {
                pos += Self::size(&n.l) + 1;
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        pos
    }

    pub fn successor(&self, key: &T) -> Option<(&T, &S)> {
        let mut cur = self.root.as_deref();
        let mut res = None;
        while let Some(n) = cur {
            if &n.key >= key {
                res = Some((&n.key, &n.value));
                cur = n.l.as_deref();
            } else {
                cur = n.r.as_deref();
            }
        }
        res
    }

    pub fn predecessor(&self, key: &T) -> Option<(&T, &S)> {
        let mut cur = self.root.as_deref();
        let mut res = None;
        while let Some(n) = cur {
            if &n.key <= key {
                res = Some((&n.key, &n.value));
                cur = n.r.as_deref();
            } else {
                cur = n.l.as_deref();
            }
        }
        res
    }

    pub fn split_off(&mut self, key: &T) -> Self {
        let root = self.root.take();
        let (l, r) = Self::split_inner(root, key);
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
            (Some((l, _)), Some((r, _))) => l < r,
            _ => true,
        });
        let l = self.root.take();
        Self::merge_inner(&mut self.root, l, rhs.root.take());
    }

    pub fn iter(&self) -> Iter<'_, T, S> {
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

    fn size(node: &Option<Box<Node<T, S>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn update(n: &mut Node<T, S>) {
        n.size = 1 + Self::size(&n.l) + Self::size(&n.r);
    }

    fn rotate_right(t: &mut Option<Box<Node<T, S>>>) {
        let mut node = t.take().unwrap();
        let mut l = node.l.take().unwrap();
        node.l = l.r.take();
        Self::update(&mut node);
        l.r = Some(node);
        Self::update(&mut l);
        *t = Some(l);
    }

    fn rotate_left(t: &mut Option<Box<Node<T, S>>>) {
        let mut node = t.take().unwrap();
        let mut r = node.r.take().unwrap();
        node.r = r.l.take();
        Self::update(&mut node);
        r.l = Some(node);
        Self::update(&mut r);
        *t = Some(r);
    }

    fn insert_inner(t: &mut Option<Box<Node<T, S>>>, mut it: Box<Node<T, S>>) -> Option<S> {
        let Some(node) = t.as_mut() else {
            *t = Some(it);
            return None;
        };
        match it.key.cmp(&node.key) {
            std::cmp::Ordering::Equal => {
                std::mem::swap(&mut node.value, &mut it.value);
                Some(it.value)
            }
            std::cmp::Ordering::Less => {
                let value = Self::insert_inner(&mut node.l, it);
                Self::update(node);
                if node.l.as_ref().unwrap().priority > node.priority {
                    Self::rotate_right(t);
                }
                value
            }
            std::cmp::Ordering::Greater => {
                let value = Self::insert_inner(&mut node.r, it);
                Self::update(node);
                if node.r.as_ref().unwrap().priority > node.priority {
                    Self::rotate_left(t);
                }
                value
            }
        }
    }

    fn remove_inner(t: &mut Option<Box<Node<T, S>>>, key: &T) -> Option<S> {
        let Some(node) = t.as_deref() else {
            return None;
        };
        match key.cmp(&node.key) {
            std::cmp::Ordering::Equal => {
                let node = t.take().unwrap();
                Self::merge_inner(t, node.l, node.r);
                Some(node.value)
            }
            std::cmp::Ordering::Less => {
                let node = t.as_mut().unwrap();
                let value = Self::remove_inner(&mut node.l, key);
                if value.is_some() {
                    Self::update(node);
                }
                value
            }
            std::cmp::Ordering::Greater => {
                let node = t.as_mut().unwrap();
                let value = Self::remove_inner(&mut node.r, key);
                if value.is_some() {
                    Self::update(node);
                }
                value
            }
        }
    }

    fn split_inner(node: Option<Box<Node<T, S>>>, key: &T) -> (Option<Box<Node<T, S>>>, Option<Box<Node<T, S>>>) {
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

    fn split_inner_index(node: Option<Box<Node<T, S>>>, k: usize) -> (Option<Box<Node<T, S>>>, Option<Box<Node<T, S>>>) {
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

    fn merge_inner(t: &mut Option<Box<Node<T, S>>>, l: Option<Box<Node<T, S>>>, r: Option<Box<Node<T, S>>>) {
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
}

pub struct Iter<'a, T: Ord, S> {
    next: Vec<&'a Node<T, S>>,
    back: Vec<&'a Node<T, S>>,
    remaining: usize,
}

impl<'a, T: Ord, S> Iterator for Iter<'a, T, S> {
    type Item = (&'a T, &'a S);

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
        Some((&node.key, &node.value))
    }
}

impl<'a, T: Ord, S> DoubleEndedIterator for Iter<'a, T, S> {
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
        Some((&node.key, &node.value))
    }
}

impl<'a, T: Ord, S> IntoIterator for &'a TreapMap<T, S> {
    type Item = (&'a T, &'a S);
    type IntoIter = Iter<'a, T, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Ord, S> FromIterator<(T, S)> for TreapMap<T, S> {
    fn from_iter<I: IntoIterator<Item = (T, S)>>(iter: I) -> Self {
        let mut treap = TreapMap::new();
        for (k, v) in iter {
            treap.insert(k, v);
        }
        treap
    }
}
