use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    child: [Option<usize>; 2],
    value: usize,
    jump: Option<usize>,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct XFastTrie {
    bits: u32,
    nodes: Vec<Node>,
    map: Vec<HashMap<usize, usize>>,
    len: usize,
    head: Option<usize>, // smallest leaf
    tail: Option<usize>, //  largest leaf
}

impl XFastTrie {
    pub fn new(bits: u32) -> Self {
        assert!(0 < bits && bits < 64);
        let root = Node {
            child: [None, None],
            value: 0,
            jump: None,
            prev: None,
            next: None,
        };
        let mut map = vec![HashMap::new(); (bits + 1) as usize];
        map[0].insert(0, 0);
        Self {
            bits,
            nodes: vec![root],
            map,
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains(&self, x: usize) -> bool {
        self.map[self.bits as usize].contains_key(&x)
    }

    pub fn min(&self) -> Option<usize> {
        self.head.map(|id| self.nodes[id].value)
    }

    pub fn max(&self) -> Option<usize> {
        self.tail.map(|id| self.nodes[id].value)
    }

    pub fn successor(&self, x: usize) -> Option<usize> {
        let Some((_, id, b)) = self.find(x) else {
            return (self.len > 0).then_some(x);
        };
        if b == 0 {
            // left missing: jump = min of right subtree = successor
            self.nodes[id].jump.map(|j| self.nodes[j].value)
        } else {
            // right missing: jump = max of left subtree (< x); successor = jump.next
            self.nodes[id].jump.and_then(|j| self.nodes[j].next).map(|n| self.nodes[n].value)
        }
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        let Some((_, id, b)) = self.find(x) else {
            return (self.len > 0).then_some(x);
        };
        if b == 1 {
            // right missing: jump = max of left subtree = predecessor
            self.nodes[id].jump.map(|j| self.nodes[j].value)
        } else {
            // left missing: jump = min of right subtree (> x); predecessor = jump.prev
            self.nodes[id].jump.and_then(|j| self.nodes[j].prev).map(|p| self.nodes[p].value)
        }
    }

    pub fn insert(&mut self, x: usize) {
        if self.contains(x) {
            return;
        }

        let succ = self.successor(x);
        let pred = self.predecessor(x);
        let mut cur = 0;
        for k in 0..self.bits {
            let b = (x >> (self.bits - k - 1)) & 1;
            let nxt = if let Some(nxt) = self.nodes[cur].child[b] {
                nxt
            } else {
                let nxt = self.nodes.len();
                self.nodes.push(Node {
                    child: [None, None],
                    jump: None,
                    value: 0,
                    prev: None,
                    next: None,
                });
                self.nodes[cur].child[b] = Some(nxt);
                let prefix = x >> (self.bits - k - 1);
                self.map[(k + 1) as usize].insert(prefix, nxt);
                nxt
            };
            cur = nxt;
        }

        let leaf_id = cur;
        let succ_id = succ.and_then(|v| self.map[self.bits as usize].get(&v).copied());
        let pred_id = pred.and_then(|v| self.map[self.bits as usize].get(&v).copied());
        self.nodes[leaf_id].value = x;
        self.nodes[leaf_id].prev = pred_id;
        self.nodes[leaf_id].next = succ_id;

        if let Some(p) = pred_id {
            self.nodes[p].next = Some(leaf_id);
        } else {
            self.head = Some(leaf_id);
        }
        if let Some(s) = succ_id {
            self.nodes[s].prev = Some(leaf_id);
        } else {
            self.tail = Some(leaf_id);
        }

        for k in 0..self.bits {
            let parent_prefix = x >> (self.bits - k);
            let parent_id = self.map[k as usize][&parent_prefix];
            let left = self.nodes[parent_id].child[0];
            let right = self.nodes[parent_id].child[1];
            match (left, right) {
                (Some(_), Some(_)) => {
                    self.nodes[parent_id].jump = None;
                }
                (None, Some(_)) => {
                    // left missing: jump = min of right subtree
                    let cur_jump = self.nodes[parent_id].jump;
                    if cur_jump.is_none() || x < self.nodes[cur_jump.unwrap()].value {
                        self.nodes[parent_id].jump = Some(leaf_id);
                    }
                }
                (Some(_), None) => {
                    // right missing: jump = max of left subtree
                    let cur_jump = self.nodes[parent_id].jump;
                    if cur_jump.is_none() || x > self.nodes[cur_jump.unwrap()].value {
                        self.nodes[parent_id].jump = Some(leaf_id);
                    }
                }
                (None, None) => unreachable!(),
            }
        }

        self.len += 1;
    }

    pub fn remove(&mut self, x: usize) -> bool {
        let Some(leaf_id) = self.map[self.bits as usize].get(&x).copied() else {
            return false;
        };
        let prev = self.nodes[leaf_id].prev;
        let next = self.nodes[leaf_id].next;
        if let Some(p) = prev {
            self.nodes[p].next = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        }
        if self.head == Some(leaf_id) {
            self.head = next;
        }
        if self.tail == Some(leaf_id) {
            self.tail = prev;
        }
        self.map[self.bits as usize].remove(&x);

        let mut child_removed = true;
        for k in (0..self.bits).rev() {
            let parent_prefix = x >> (self.bits - k);
            let parent_id = self.map[k as usize][&parent_prefix];
            let b = (x >> (self.bits - k - 1)) & 1;

            if child_removed {
                self.nodes[parent_id].child[b] = None;
                if self.nodes[parent_id].child[b ^ 1].is_none() {
                    if k > 0 {
                        self.map[k as usize].remove(&parent_prefix);
                    } else {
                        self.nodes[parent_id].jump = None;
                    }
                    continue;
                }
                child_removed = false;
                if b == 0 {
                    // left missing: jump = min of right subtree = next
                    self.nodes[parent_id].jump = next;
                } else {
                    // right missing: jump = max of left subtree = prev
                    self.nodes[parent_id].jump = prev;
                }
            } else {
                if self.nodes[parent_id].jump == Some(leaf_id) {
                    if self.nodes[parent_id].child[0].is_none() {
                        // left missing: jump = min of right subtree = next
                        self.nodes[parent_id].jump = next;
                    } else {
                        // right missing: jump = max of left subtree = prev
                        self.nodes[parent_id].jump = prev;
                    }
                }
            }
        }

        self.len -= 1;
        true
    }

    fn find(&self, x: usize) -> Option<(u32, usize, usize)> {
        if self.len == 0 {
            return None;
        }
        let mut hi = self.bits;
        let mut lo = 0;
        while lo < hi {
            let mi = (lo + hi + 1) / 2;
            let prefix = x >> (self.bits - mi);
            if self.map[mi as usize].contains_key(&prefix) {
                lo = mi;
            } else {
                hi = mi - 1;
            }
        }
        if lo == self.bits {
            return None;
        }
        let prefix = x >> (self.bits - lo);
        let id = self.map[lo as usize][&prefix];
        let b = (x >> (self.bits - lo - 1)) & 1;
        Some((lo, id, b))
    }
}
