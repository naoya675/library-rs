use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct IntervalSet<T, VAL> {
    identity: VAL,
    map: BTreeMap<T, (T, VAL)>, // l -> (r, val), representing [l, r)
}

impl<T, VAL> IntervalSet<T, VAL>
where
    T: Ord + Copy,
    VAL: Clone + PartialEq,
{
    pub fn new(identity: VAL) -> Self {
        Self {
            identity,
            map: BTreeMap::new(),
        }
    }

    pub fn from_vec(v: &[VAL], identity: VAL) -> Self
    where
        T: From<usize>,
    {
        let mut set = Self::new(identity);
        let mut i = 0;
        while i < v.len() {
            let mut j = i;
            while j < v.len() && v[i] == v[j] {
                j += 1;
            }
            set.map.insert(T::from(i), (T::from(j), v[i].clone()));
            i = j;
        }
        set
    }

    pub fn get(&self, p: T) -> Option<(T, T, VAL)> {
        let (&l, (r, val)) = self.map.range(..=p).next_back()?;
        if p < *r { Some((l, *r, val.clone())) } else { None }
    }

    pub fn lower_bound(&self, p: T) -> Option<(T, T, VAL)> {
        if let Some(result) = self.get(p) {
            return Some(result);
        }
        let (&l, (r, val)) = self.map.range((std::ops::Bound::Excluded(p), std::ops::Bound::Unbounded)).next()?;
        Some((l, *r, val.clone()))
    }

    pub fn upper_bound(&self, p: T) -> Option<(T, T, VAL)> {
        let (&l, (r, val)) = self.map.range((std::ops::Bound::Excluded(p), std::ops::Bound::Unbounded)).next()?;
        Some((l, *r, val.clone()))
    }

    pub fn covered_point(&self, p: T) -> bool {
        self.get(p).is_some()
    }

    pub fn covered_range(&self, l: T, r: T) -> bool {
        assert!(l <= r);
        if l == r {
            return true;
        }
        if let Some((_, ir, _)) = self.get(l) { r <= ir } else { false }
    }

    pub fn same(&self, p: T, q: T) -> bool {
        match (self.get(p), self.get(q)) {
            (Some((lp, _, _)), Some((lq, _, _))) => lp == lq,
            _ => false,
        }
    }

    pub fn get_val(&self, p: T) -> VAL {
        if let Some((_, _, val)) = self.get(p) { val } else { self.identity.clone() }
    }

    pub fn get_mex(&self, p: T) -> T {
        if let Some((_, r, _)) = self.get(p) { r } else { p }
    }

    pub fn update(&mut self, l: T, r: T, val: VAL) {
        self.update_inner(l, r, val, |_, _, _| {}, |_, _, _| {});
    }

    // update [l, r) with value val
    pub fn update_inner<F, G>(&mut self, mut l: T, mut r: T, val: VAL, mut add: F, mut del: G)
    where
        F: FnMut(T, T, &VAL),
        G: FnMut(T, T, &VAL),
    {
        assert!(l <= r);
        if l == r {
            return;
        }

        // forward pass: process entries with key in [l, r]
        let keys: Vec<T> = self.map.range(l..=r).map(|(&k, _)| k).collect();
        for key in keys {
            // adjacency check: entry starts exactly at r
            if key == r {
                if let Some((nr, nval)) = self.map.get(&key) {
                    if *nval == val {
                        let nr = *nr;
                        let nval = nval.clone();
                        r = nr;
                        del(key, nr, &nval);
                        self.map.remove(&key);
                    }
                }
                break;
            }
            let (nr, nval) = self.map.remove(&key).unwrap();
            if nr <= r {
                // fully contained in [l, r)
                del(key, nr, &nval);
            } else {
                if nval == val {
                    // extend r
                    r = nr;
                    del(key, nr, &nval);
                } else {
                    // split, reinsert [r, nr)
                    del(key, nr, &nval);
                    self.map.insert(r, (nr, nval.clone()));
                    add(r, nr, &nval);
                }
            }
        }

        // backward pass: last entry with key <= l
        let prev = self.map.range(..=l).next_back().map(|(&k, (r, v))| (k, *r, v.clone()));
        if let Some((nl, nr, nval)) = prev {
            if nr == l {
                // adjacent on the left
                if nval == val {
                    l = nl;
                    del(nl, nr, &nval);
                    self.map.remove(&nl);
                }
            } else if l < nr {
                // overlaps with [l, r)
                if nval == val {
                    // merge
                    l = l.min(nl);
                    r = r.max(nr);
                    del(nl, nr, &nval);
                    self.map.remove(&nl);
                } else {
                    // split, reinsert right piece [r, nr)
                    if r < nr {
                        self.map.insert(r, (nr, nval.clone()));
                        add(r, nr, &nval);
                    }
                    del(nl, nr, &nval);
                    self.map.remove(&nl);
                    // reinsert left piece [nl, l)
                    self.map.insert(nl, (l, nval.clone()));
                    add(nl, l, &nval);
                }
            }
        }

        // insert the new interval [l, r)
        self.map.insert(l, (r, val.clone()));
        add(l, r, &val);
    }

    pub fn insert(&mut self, l: T, r: T) {
        self.update_inner(l, r, self.identity.clone(), |_, _, _| {}, |_, _, _| {});
    }

    pub fn insert_inner<F, G>(&mut self, l: T, r: T, add: F, del: G)
    where
        F: FnMut(T, T, &VAL),
        G: FnMut(T, T, &VAL),
    {
        self.update_inner(l, r, self.identity.clone(), add, del);
    }

    pub fn erase(&mut self, l: T, r: T) {
        self.erase_inner(l, r, |_, _, _| {}, |_, _, _| {});
    }

    // erase [l, r)
    pub fn erase_inner<F, G>(&mut self, l: T, r: T, mut add: F, mut del: G)
    where
        F: FnMut(T, T, &VAL),
        G: FnMut(T, T, &VAL),
    {
        assert!(l <= r);
        if l == r {
            return;
        }

        // forward pass: process entries with key in [l, r)
        let keys: Vec<T> = self.map.range(l..r).map(|(&k, _)| k).collect();
        for key in keys {
            let (nr, nval) = self.map.remove(&key).unwrap();
            if nr <= r {
                // fully contained in [l, r)
                del(key, nr, &nval);
            } else {
                // extends beyond r: split, reinsert [r, nr)
                del(key, nr, &nval);
                self.map.insert(r, (nr, nval.clone()));
                add(r, nr, &nval);
            }
        }

        // backward pass: last entry with key < l
        let prev = self.map.range(..l).next_back().map(|(&k, (r, v))| (k, *r, v.clone()));
        if let Some((nl, nr, nval)) = prev {
            if l < nr {
                // split, reinsert right piece [r, nr)
                if r < nr {
                    self.map.insert(r, (nr, nval.clone()));
                    add(r, nr, &nval);
                }
                del(nl, nr, &nval);
                self.map.remove(&nl);
                // reinsert left piece [nl, l)
                self.map.insert(nl, (l, nval.clone()));
                add(nl, l, &nval);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (T, T, VAL)> + '_ {
        self.map.iter().map(|(&l, (r, val))| (l, *r, val.clone()))
    }
}

impl<T, VAL> std::fmt::Display for IntervalSet<T, VAL>
where
    T: Copy + std::fmt::Display,
    VAL: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (&l, (r, val)) in &self.map {
            write!(f, "([{}, {}): {}) ", l, r, val)?;
        }
        Ok(())
    }
}
