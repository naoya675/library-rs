use std::collections::HashMap;

use treap::Treap;
use x_fast_trie::XFastTrie;

#[derive(Debug, Clone)]
pub struct YFastTrie {
    bits: u32,
    reps: XFastTrie,                    // representatives (min of each map)
    maps: HashMap<usize, Treap<usize>>, // reps -> maps
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
            self.maps[&r].contains(x)
        } else {
            false
        }
    }

    pub fn min(&self) -> Option<usize> {
        self.reps.min()
    }

    pub fn max(&self) -> Option<usize> {
        self.reps.max().and_then(|r| self.maps[&r].max())
    }

    pub fn successor(&self, x: usize) -> Option<usize> {
        if let Some(r) = self.reps.predecessor(x) {
            if let Some(y) = self.maps[&r].successor(x) {
                return Some(y);
            }
        }
        self.reps.successor(x)
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        if let Some(r) = self.reps.predecessor(x) {
            if let Some(y) = self.maps[&r].predecessor(x) {
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
            let mut map = Treap::new();
            map.insert(x);
            self.maps.insert(x, map);
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
        map.remove(x);
        if map.is_empty() {
            self.reps.remove(target);
            self.len -= 1;
            return true;
        }
        let rep = map.min().unwrap();
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

    fn split_map(&mut self, rep: usize, mut map: Treap<usize>) -> (usize, usize) {
        let r_map = map.split_off_at(map.len() / 2);
        let mid = r_map.min().unwrap();
        self.maps.insert(rep, map);
        self.maps.insert(mid, r_map);
        (rep, mid)
    }

    fn merge_map(&mut self, rep: usize, mut map: Treap<usize>) -> (usize, Treap<usize>) {
        if let Some(r) = self.reps.successor(rep + 1) {
            let r_map = self.maps.remove(&r).unwrap();
            map.merge(r_map);
            self.reps.remove(r);
            return (rep, map);
        }
        if let Some(l) = if rep > 0 { self.reps.predecessor(rep - 1) } else { None } {
            let mut l_map = self.maps.remove(&l).unwrap();
            l_map.merge(map);
            self.reps.remove(rep);
            return (l, l_map);
        }
        (rep, map)
    }
}
