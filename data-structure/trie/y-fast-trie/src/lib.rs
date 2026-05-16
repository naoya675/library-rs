use std::collections::{BTreeSet, HashMap};

use x_fast_trie::XFastTrie;

#[derive(Debug, Clone)]
pub struct YFastTrie {
    bits: u32,
    reps: XFastTrie,                       // representatives (min of each map)
    maps: HashMap<usize, BTreeSet<usize>>, // reps -> maps; consider replacing BTreeSet with Treap
    len: usize,
}

impl YFastTrie {
    pub fn new(bits: u32) -> Self {
        assert!(0 < bits && bits < 64);
        Self {
            bits,
            reps: XFastTrie::new(bits),
            maps: HashMap::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains(&self, x: usize) -> bool {
        if let Some(r) = self.reps.predecessor(x) {
            self.maps[&r].contains(&x)
        } else {
            false
        }
    }

    pub fn min(&self) -> Option<usize> {
        self.reps.min()
    }

    pub fn max(&self) -> Option<usize> {
        self.reps.max().map(|r| *self.maps[&r].iter().next_back().unwrap())
    }

    pub fn successor(&self, x: usize) -> Option<usize> {
        if let Some(r) = self.reps.predecessor(x) {
            if let Some(&y) = self.maps[&r].range(x..).next() {
                return Some(y);
            }
        }
        self.reps.successor(x)
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        if let Some(r) = self.reps.predecessor(x) {
            if let Some(&y) = self.maps[&r].range(..=x).next_back() {
                return Some(y);
            }
        }
        None
    }

    pub fn insert(&mut self, x: usize) {
        if self.contains(x) {
            return;
        }

        if self.reps.len() == 0 {
            self.reps.insert(x);
            self.maps.insert(x, BTreeSet::from([x]));
            self.len += 1;
            return;
        }

        let (rep, map) = if let Some(target) = self.reps.predecessor(x) {
            let mut map = self.maps.remove(&target).unwrap();
            map.insert(x);
            (target, map)
        } else {
            let target = self.reps.min().unwrap();
            let mut map = self.maps.remove(&target).unwrap();
            map.insert(x);
            self.reps.remove(target);
            self.reps.insert(x);
            (x, map)
        };

        let upper = (self.bits + self.bits) as usize;
        if map.len() > upper {
            let (_, r_rep) = self.split_map(rep, map);
            self.reps.insert(r_rep);
        } else {
            self.maps.insert(rep, map);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, x: usize) -> bool {
        if !self.contains(x) {
            return false;
        }

        let Some(target) = self.reps.predecessor(x) else {
            return false;
        };
        let mut map = self.maps.remove(&target).unwrap();
        map.remove(&x);
        if map.is_empty() {
            self.reps.remove(target);
            self.len -= 1;
            return true;
        }
        let rep = *map.iter().next().unwrap();
        if rep != target {
            self.reps.remove(target);
            self.reps.insert(rep);
        }

        let lower = (self.bits / 2) as usize;
        let (rep, map) = if map.len() < lower { self.merge_map(rep, map) } else { (rep, map) };

        let upper = (self.bits + self.bits) as usize;
        if map.len() > upper {
            let (_, r_rep) = self.split_map(rep, map);
            self.reps.insert(r_rep);
        } else {
            self.maps.insert(rep, map);
        }
        self.len -= 1;
        true
    }

    fn split_map(&mut self, rep: usize, map: BTreeSet<usize>) -> (usize, usize) {
        let mid = *map.iter().nth(map.len() / 2).unwrap();
        let l_map: BTreeSet<usize> = map.range(..mid).copied().collect();
        let r_map: BTreeSet<usize> = map.range(mid..).copied().collect();
        self.maps.insert(rep, l_map);
        self.maps.insert(mid, r_map);
        (rep, mid)
    }

    fn merge_map(&mut self, rep: usize, mut map: BTreeSet<usize>) -> (usize, BTreeSet<usize>) {
        let right = if rep < usize::MAX { self.reps.successor(rep + 1) } else { None };
        if let Some(r) = right {
            let r_map = self.maps.remove(&r).unwrap();
            map.extend(r_map);
            self.reps.remove(r);
            return (rep, map);
        }
        let left = if rep > 0 { self.reps.predecessor(rep - 1) } else { None };
        if let Some(l) = left {
            let l_map = self.maps.remove(&l).unwrap();
            let mut merged = l_map;
            merged.extend(map);
            self.reps.remove(rep);
            return (l, merged);
        }
        (rep, map)
    }
}
