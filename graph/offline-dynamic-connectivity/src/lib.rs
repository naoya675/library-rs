use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OfflineDynamicConnectivity {
    n: usize,
    q: usize,
    size: usize,
    tree: Vec<Vec<(usize, usize)>>,
    qadd: Vec<Vec<(usize, usize)>>,
    qdel: Vec<Vec<(usize, usize)>>,
}

impl OfflineDynamicConnectivity {
    pub fn new(n: usize, q: usize) -> Self {
        let size = q.next_power_of_two();
        Self {
            n,
            q,
            size,
            tree: vec![vec![]; 2 * size],
            qadd: vec![vec![]; q],
            qdel: vec![vec![]; q],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, l: usize, r: usize) {
        assert!(u < self.n);
        assert!(v < self.n);
        assert!(l <= r && r <= self.q);
        self.add_edge_inner((u.min(v), u.max(v)), l, r);
    }

    pub fn insert(&mut self, t: usize, u: usize, v: usize) {
        assert!(t < self.q);
        assert!(u < self.n);
        assert!(v < self.n);
        self.qadd[t].push((u.min(v), u.max(v)));
    }

    pub fn remove(&mut self, t: usize, u: usize, v: usize) {
        assert!(t < self.q);
        assert!(u < self.n);
        assert!(v < self.n);
        self.qdel[t].push((u.min(v), u.max(v)));
    }

    pub fn run<S, F>(&mut self, state: &mut S, merge: fn(&mut S, usize, usize), rollback: fn(&mut S), mut f: F)
    where
        F: FnMut(&mut S, usize),
    {
        self.build();
        self.dfs(state, merge, rollback, &mut f, 1, 0, self.size);
    }

    fn build(&mut self) {
        let mut map = HashMap::new();
        let mut ranges = vec![];
        for t in 0..self.q {
            for &e in &self.qadd[t] {
                let (appeared, count) = map.entry(e).or_insert((t, 0));
                if *count == 0 {
                    *appeared = t;
                }
                *count += 1;
            }
            for &e in &self.qdel[t] {
                let (appeared, count) = map.get_mut(&e).unwrap();
                *count -= 1;
                if *count == 0 {
                    ranges.push((e, *appeared, t));
                }
            }
        }
        let mut remaining = map.into_iter().filter(|&(_, (_, count))| count > 0).collect::<Vec<_>>();
        remaining.sort();
        for &(e, (l, _)) in &remaining {
            ranges.push((e, l, self.q));
        }

        for &(e, l, r) in &ranges {
            self.add_edge_inner(e, l, r);
        }
    }

    fn dfs<S, F>(&self, state: &mut S, merge: fn(&mut S, usize, usize), rollback: fn(&mut S), f: &mut F, k: usize, l: usize, r: usize)
    where
        F: FnMut(&mut S, usize),
    {
        if self.q <= l {
            return;
        }
        for &(u, v) in &self.tree[k] {
            merge(state, u, v);
        }
        if l + 1 == r {
            f(state, l);
        } else {
            let m = (l + r) / 2;
            self.dfs(state, merge, rollback, f, 2 * k, l, m);
            self.dfs(state, merge, rollback, f, 2 * k + 1, m, r);
        }
        for _ in 0..self.tree[k].len() {
            rollback(state);
        }
    }

    fn add_edge_inner(&mut self, e: (usize, usize), l: usize, r: usize) {
        let mut l = l + self.size;
        let mut r = r + self.size;
        while l < r {
            if l & 1 == 1 {
                self.tree[l].push(e);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.tree[r].push(e);
            }
            l >>= 1;
            r >>= 1;
        }
    }
}
