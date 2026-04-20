#[derive(Debug, Clone)]
pub struct Mo {
    n: usize,
    w: usize,
    lr: Vec<(usize, usize)>,
}

impl Mo {
    pub fn new(n: usize, q: usize) -> Self {
        let w = std::cmp::max(1, n / std::cmp::max(1, (q as f64 * 2.0 / 3.0).sqrt() as usize));
        Self { n, w, lr: Vec::new() }
    }

    pub fn add_query(&mut self, l: usize, r: usize) {
        assert!(l < r && r <= self.n);
        self.lr.push((l, r));
    }

    pub fn run_queries<AL, AR, EL, ER, Q>(&self, mut add_left: AL, mut add_right: AR, mut erase_left: EL, mut erase_right: ER, mut query: Q)
    where
        AL: FnMut(usize),
        AR: FnMut(usize),
        EL: FnMut(usize),
        ER: FnMut(usize),
        Q: FnMut(usize),
    {
        let ord = self.sort_queries();
        let mut l = 0;
        let mut r = 0;
        for &idx in &ord {
            while l > self.lr[idx].0 {
                l -= 1;
                add_left(l);
            }
            while r < self.lr[idx].1 {
                add_right(r);
                r += 1;
            }
            while l < self.lr[idx].0 {
                erase_left(l);
                l += 1;
            }
            while r > self.lr[idx].1 {
                r -= 1;
                erase_right(r);
            }
            query(idx);
        }
    }

    pub fn run_queries_simple<A, E, Q>(&self, mut add: A, mut erase: E, mut query: Q)
    where
        A: FnMut(usize),
        E: FnMut(usize),
        Q: FnMut(usize),
    {
        let ord = self.sort_queries();
        let mut l = 0;
        let mut r = 0;
        for &idx in &ord {
            while l > self.lr[idx].0 {
                l -= 1;
                add(l);
            }
            while r < self.lr[idx].1 {
                add(r);
                r += 1;
            }
            while l < self.lr[idx].0 {
                erase(l);
                l += 1;
            }
            while r > self.lr[idx].1 {
                r -= 1;
                erase(r);
            }
            query(idx);
        }
    }

    fn sort_queries(&self) -> Vec<usize> {
        let q = self.lr.len();
        let mut ord: Vec<usize> = (0..q).collect();
        let w = self.w;
        ord.sort_by(|&a, &b| {
            let block_a = self.lr[a].0 / w;
            let block_b = self.lr[b].0 / w;
            if block_a != block_b {
                block_a.cmp(&block_b)
            } else if block_a & 1 == 1 {
                self.lr[a].1.cmp(&self.lr[b].1)
            } else {
                self.lr[b].1.cmp(&self.lr[a].1)
            }
        });
        ord
    }
}
