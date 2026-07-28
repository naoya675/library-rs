use union_find::UnionFind;

#[derive(Debug, Clone)]
pub struct RangeParallelUnionFind {
    n: usize,
    uf: Vec<UnionFind>,
}

impl RangeParallelUnionFind {
    pub fn new(n: usize) -> Self {
        let log = n.next_power_of_two().ilog2().max(1) as usize;
        Self {
            n,
            uf: (0..log).map(|k| UnionFind::new(n + 1 - (1 << k))).collect(),
        }
    }

    // merges [l1, r1) and [l2, r2) elementwise
    pub fn merge<F>(&mut self, l1: usize, r1: usize, l2: usize, r2: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        assert!(l1 <= r1 && r1 <= self.n);
        assert!(l2 <= r2 && r2 <= self.n);
        assert!(r1 - l1 == r2 - l2);
        let len = r1 - l1;
        if len == 0 {
            return;
        }
        if len == 1 {
            self.merge_block(0, l1, l2, &mut f);
            return;
        }
        let k = (len - 1).ilog2() as usize;
        self.merge_block(k, l1, l2, &mut f);
        self.merge_block(k, r1 - (1 << k), r2 - (1 << k), &mut f);
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.uf[0].same(x, y)
    }

    pub fn leader(&mut self, x: usize) -> usize {
        self.uf[0].leader(x)
    }

    pub fn size(&mut self, x: usize) -> usize {
        self.uf[0].size(x)
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        self.uf[0].groups()
    }

    // merges [l1, l1 + 2^k) and [l2, l2 + 2^k) elementwise
    fn merge_block<F>(&mut self, k: usize, l1: usize, l2: usize, f: &mut F)
    where
        F: FnMut(usize, usize),
    {
        if k == 0 {
            let a = self.uf[0].leader(l1);
            let b = self.uf[0].leader(l2);
            if a == b {
                return;
            }
            let c = self.uf[0].merge(a, b);
            f(c, if c == a { b } else { a });
            return;
        }
        if self.uf[k].same(l1, l2) {
            return;
        }
        self.uf[k].merge(l1, l2);
        self.merge_block(k - 1, l1, l2, f);
        self.merge_block(k - 1, l1 + (1 << (k - 1)), l2 + (1 << (k - 1)), f);
    }
}
