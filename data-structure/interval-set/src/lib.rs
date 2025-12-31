use std::collections::BTreeSet;

pub trait RangeTrait {
    fn max_value() -> Self;
    fn min_value() -> Self;
}

macro_rules! impl_range_trait {
    ($($type:ty),* $(,)?) => {
        $(
            impl RangeTrait for $type {
                fn max_value() -> Self { <$type>::MAX }
                fn min_value() -> Self { <$type>::MIN }
            }
        )*
    };
}

impl_range_trait!(u32, i32, u64, i64, usize, isize);

#[derive(Clone, Debug)]
pub struct Node<T, VAL> {
    pub l: T,
    pub r: T,
    pub val: VAL,
}

impl<T, VAL> Node<T, VAL> {
    pub fn new(l: T, r: T, val: VAL) -> Self {
        Self { l, r, val }
    }
}

impl<T: Ord, VAL> PartialEq for Node<T, VAL> {
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.r == other.r
    }
}
impl<T: Ord, VAL> Eq for Node<T, VAL> {}

impl<T: Ord, VAL> PartialOrd for Node<T, VAL> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord, VAL> Ord for Node<T, VAL> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.l.cmp(&other.l) {
            std::cmp::Ordering::Equal => self.r.cmp(&other.r),
            ord => ord,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntervalSet<T, VAL>
where
    T: Copy + Ord + RangeTrait + Default + std::fmt::Debug,
    VAL: Clone + PartialEq + Eq + Default + std::fmt::Debug,
{
    identity: VAL,
    set: BTreeSet<Node<T, VAL>>,
}

impl<T, VAL> IntervalSet<T, VAL>
where
    T: Ord + Copy + RangeTrait + Default + std::fmt::Debug,
    VAL: Clone + PartialEq + Eq + Default + std::fmt::Debug,
{
    pub fn new(identity: VAL) -> Self {
        Self {
            identity,
            set: BTreeSet::new(),
        }
    }

    pub fn from_vec(v: &[VAL], identity: VAL) -> Self
    where
        T: From<usize>,
    {
        let mut set = IntervalSet::new(identity);
        let mut i = 0;
        while i < v.len() {
            let mut j = i;
            while j < v.len() && v[i] == v[j] {
                j += 1;
            }
            let node = Node::new(T::from(i), T::from(j), v[i].clone());
            set.set.insert(node);
            i = j;
        }
        set
    }

    pub fn get(&self, p: T) -> Option<(T, T, VAL)> {
        let key = Node::new(p, T::max_value(), VAL::default());
        if let Some(node) = self.set.range(..=key).next_back() {
            if node.l <= p && p < node.r {
                return Some((node.l, node.r, node.val.clone()));
            }
        }
        None
    }

    pub fn lower_bound(&self, p: T) -> Option<(T, T, VAL)> {
        if let Some((l, r, val)) = self.get(p) {
            return Some((l, r, val));
        }
        let key = Node::new(p, T::min_value(), VAL::default());
        if let Some(node) = self.set.range(key..).next() {
            return Some((node.l, node.r, node.val.clone()));
        }
        None
    }

    pub fn covered_point(&self, p: T) -> bool {
        if let Some((il, ir, v)) = self.get(p) {
            return true;
        }
        false
    }

    pub fn covered_range(&self, l: T, r: T) -> bool {
        assert!(l <= r);
        if l == r {
            return true;
        }
        if let Some((il, ir, v)) = self.get(l) {
            return r <= ir;
        }
        false
    }

    pub fn same(&self, p: T, q: T) -> bool {
        if let (Some(pnode), Some(qnode)) = (self.get(p), self.get(q)) {
            pnode == qnode
        } else {
            false
        }
    }

    pub fn get_val(&self, p: T) -> VAL {
        if let Some((_, _, v)) = self.get(p) {
            return v;
        }
        self.identity.clone()
    }

    pub fn get_mex(&self, p: T) -> T {
        let key = Node::new(p, T::max_value(), VAL::default());
        if let Some(node) = self.set.range(..=key).next_back() {
            if node.l <= p && p < node.r {
                return node.r;
            }
        }
        p
    }

    pub fn inner_update<F, G>(&mut self, mut l: T, mut r: T, val: VAL, mut add: F, mut del: G)
    where
        F: FnMut(T, T, &VAL),
        G: FnMut(T, T, &VAL),
    {
        assert!(l <= r);
        if l == r {
            return;
        }

        let lkey = Node::new(l, T::min_value(), val.clone());
        let rkey = Node::new(r, T::max_value(), val.clone());
        for node in self.set.range(lkey..rkey).cloned().collect::<Vec<_>>() {
            if node.l == r {
                if node.val == val {
                    r = node.r;
                    del(node.l, node.r, &node.val);
                    let _ = self.set.take(&node);
                    // self.set.remove(&node);
                }
                break;
            }
            if node.r <= r {
                del(node.l, node.r, &node.val);
                let _ = self.set.take(&node);
            } else {
                if node.val == val {
                    r = node.r;
                    del(node.l, node.r, &node.val);
                    let _ = self.set.take(&node);
                } else {
                    // split, reinsert [r, node.r)
                    del(node.l, node.r, &node.val);
                    let _ = self.set.take(&node);
                    let rnode = Node::new(r, node.r, node.val.clone());
                    self.set.insert(rnode.clone());
                    add(rnode.l, rnode.r, &rnode.val);
                }
            }
        }

        let key = Node::new(l, T::max_value(), val.clone());
        if let Some(node) = self.set.range(..=key).next_back().cloned() {
            if node.r == l {
                if node.val == val {
                    l = node.l;
                    del(node.l, node.r, &node.val);
                    let _ = self.set.take(&node);
                }
            } else if l < node.r {
                if node.val == val {
                    // merge
                    l = l.min(node.l);
                    r = r.max(node.r);
                    del(node.l, node.r, &node.val);
                    let _ = self.set.take(&node);
                } else {
                    // split, reinsert [r, node.r)
                    if r < node.r {
                        let rnode = Node::new(r, node.r, node.val.clone());
                        self.set.insert(rnode.clone());
                        add(rnode.l, rnode.r, &rnode.val);
                    }
                    del(node.l, node.r, &node.val);
                    let _ = self.set.take(&node);
                    let lnode = Node::new(node.l, l, node.val.clone());
                    self.set.insert(lnode.clone());
                    add(lnode.l, lnode.r, &lnode.val);
                }
            }
        }
        let nnode = Node::new(l, r, val.clone());
        self.set.insert(nnode.clone());
        add(nnode.l, nnode.r, &nnode.val);
    }

    pub fn update(&mut self, l: T, r: T, val: VAL) {
        self.inner_update(l, r, val, |_l, _r, _v| {}, |_l, _r, _v| {});
    }

    pub fn insert(&mut self, l: T, r: T) {
        self.inner_update(l, r, self.identity.clone(), |_l, _r, _v| {}, |_l, _r, _v| {});
    }

    pub fn inner_erase<F, G>(&mut self, l: T, r: T, mut add: F, mut del: G)
    where
        F: FnMut(T, T, &VAL),
        G: FnMut(T, T, &VAL),
    {
        assert!(l <= r);
        if l == r {
            return;
        }

        let lkey = Node::new(l, T::min_value(), self.identity.clone());
        let rkey = Node::new(r, T::max_value(), self.identity.clone());
        for node in self.set.range(lkey..rkey).cloned().collect::<Vec<_>>() {
            if node.l == r {
                break;
            }
            if node.r <= r {
                del(node.l, node.r, &node.val);
                let _ = self.set.take(&node);
            } else {
                del(node.l, node.r, &node.val);
                let _ = self.set.take(&node);
                let rnode = Node::new(r, node.r, node.val.clone());
                self.set.insert(rnode.clone());
                add(rnode.l, rnode.r, &rnode.val);
            }
        }

        let key = Node::new(l, T::max_value(), self.identity.clone());
        if let Some(node) = self.set.range(..=key).next_back().cloned() {
            if l < node.r {
                if r < node.r {
                    let rnode = Node::new(r, node.r, node.val.clone());
                    self.set.insert(rnode.clone());
                    add(rnode.l, rnode.r, &rnode.val);
                }
                del(node.l, node.r, &node.val);
                let _ = self.set.take(&node);
                let lnode = Node::new(node.l, l, node.val.clone());
                self.set.insert(lnode.clone());
                add(lnode.l, lnode.r, &lnode.val);
            }
        }
    }

    pub fn erase(&mut self, l: T, r: T) {
        self.inner_erase(l, r, |_l, _r, _v| {}, |_l, _r, _v| {});
    }

    pub fn iter(&self) -> impl Iterator<Item = (T, T, VAL)> + '_ {
        self.set.iter().map(|node| (node.l, node.r, node.val.clone()))
    }
}
