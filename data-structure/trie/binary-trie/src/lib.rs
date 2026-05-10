#[derive(Debug, Clone)]
struct Node {
    child: [Option<usize>; 2],
    count: usize,
}

#[derive(Debug, Clone)]
pub struct BinaryTrie {
    bits: u32,
    nodes: Vec<Node>,
}

impl BinaryTrie {
    pub fn new(bits: u32) -> Self {
        Self {
            bits,
            nodes: vec![Node { child: [None, None], count: 0 }],
        }
    }

    pub fn len(&self) -> usize {
        self.nodes[0].count
    }

    pub fn insert(&mut self, x: usize) {
        self.insert_inner(x, 1);
    }

    pub fn remove(&mut self, x: usize) -> bool {
        self.remove_inner(x, 1)
    }

    pub fn contains(&self, x: usize) -> bool {
        self.count(x) > 0
    }

    pub fn count(&self, x: usize) -> usize {
        let mut cur = 0;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            if let Some(nxt) = self.nodes[cur].child[b] {
                cur = nxt;
            } else {
                return 0;
            }
        }
        self.nodes[cur].count
    }

    pub fn kth(&self, mut k: usize) -> usize {
        assert!(k < self.len());
        let mut cur = 0;
        let mut x = 0;
        for i in (0..self.bits).rev() {
            let lhs_count = if let Some(nxt) = self.nodes[cur].child[0] { self.nodes[nxt].count } else { 0 };
            if k < lhs_count {
                cur = self.nodes[cur].child[0].unwrap();
            } else {
                k -= lhs_count;
                cur = self.nodes[cur].child[1].unwrap();
                x |= 1 << i;
            }
        }
        x
    }

    pub fn min(&self) -> usize {
        self.kth(0)
    }

    pub fn max(&self) -> usize {
        self.kth(self.len() - 1)
    }

    pub fn xor_min(&self, x: usize) -> usize {
        assert!(self.len() > 0);
        let mut cur = 0;
        let mut y = 0;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            let (nxt, bit) = self.descend(cur, b).or_else(|| self.descend(cur, b ^ 1)).unwrap();
            cur = nxt;
            y |= bit << i;
        }
        x ^ y
    }

    pub fn xor_max(&self, x: usize) -> usize {
        assert!(self.len() > 0);
        let mut cur = 0;
        let mut y = 0;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            let (nxt, bit) = self.descend(cur, b ^ 1).or_else(|| self.descend(cur, b)).unwrap();
            cur = nxt;
            y |= bit << i;
        }
        x ^ y
    }

    pub fn lower_bound(&self, x: usize) -> usize {
        let mut cur = 0;
        let mut res = 0;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            if b == 1 {
                if let Some(nxt) = self.nodes[cur].child[0] {
                    res += self.nodes[nxt].count;
                }
            }
            if let Some(nxt) = self.nodes[cur].child[b] {
                cur = nxt;
            } else {
                return res;
            }
        }
        res
    }

    pub fn upper_bound(&self, x: usize) -> usize {
        let mut cur = 0;
        let mut res = 0;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            if b == 1 {
                if let Some(nxt) = self.nodes[cur].child[0] {
                    res += self.nodes[nxt].count;
                }
            }
            if let Some(nxt) = self.nodes[cur].child[b] {
                cur = nxt;
            } else {
                return res;
            }
        }
        res + self.nodes[cur].count
    }

    /*
     *
    pub fn successor(&self, x: usize) -> Option<usize> {
        let pos = self.lower_bound(x);
        if pos < self.len() { Some(self.kth(pos)) } else { None }
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        let pos = self.upper_bound(x);
        if pos == 0 { None } else { Some(self.kth(pos - 1)) }
    }
     */

    fn insert_inner(&mut self, x: usize, w: usize) {
        let mut cur = 0;
        self.nodes[cur].count += w;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            let nxt = if let Some(nxt) = self.nodes[cur].child[b] {
                nxt
            } else {
                let nxt = self.nodes.len();
                self.nodes.push(Node { child: [None, None], count: 0 });
                self.nodes[cur].child[b] = Some(nxt);
                nxt
            };
            cur = nxt;
            self.nodes[cur].count += w;
        }
    }

    fn remove_inner(&mut self, x: usize, w: usize) -> bool {
        if self.count(x) < w {
            return false;
        }
        let mut cur = 0;
        self.nodes[cur].count -= w;
        for i in (0..self.bits).rev() {
            let b = (x >> i) & 1;
            let nxt = self.nodes[cur].child[b].unwrap();
            cur = nxt;
            self.nodes[cur].count -= w;
        }
        true
    }

    fn descend(&self, v: usize, b: usize) -> Option<(usize, usize)> {
        self.nodes[v].child[b].filter(|&nxt| self.nodes[nxt].count > 0).map(|nxt| (nxt, b))
    }
}
