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

    pub fn run_queries<S, AL, AR, EL, ER, Q>(&self, state: &mut S, mut add_left: AL, mut add_right: AR, mut erase_left: EL, mut erase_right: ER, mut query: Q)
    where
        AL: FnMut(&mut S, usize),
        AR: FnMut(&mut S, usize),
        EL: FnMut(&mut S, usize),
        ER: FnMut(&mut S, usize),
        Q: FnMut(&mut S, usize),
    {
        let ord = self.sort_queries();
        let mut l = 0;
        let mut r = 0;
        for &idx in &ord {
            while l > self.lr[idx].0 {
                l -= 1;
                add_left(state, l);
            }
            while r < self.lr[idx].1 {
                add_right(state, r);
                r += 1;
            }
            while l < self.lr[idx].0 {
                erase_left(state, l);
                l += 1;
            }
            while r > self.lr[idx].1 {
                r -= 1;
                erase_right(state, r);
            }
            query(state, idx);
        }
    }

    pub fn run_queries_simple<S, A, E, Q>(&self, state: &mut S, mut add: A, mut erase: E, mut query: Q)
    where
        A: FnMut(&mut S, usize),
        E: FnMut(&mut S, usize),
        Q: FnMut(&mut S, usize),
    {
        let ord = self.sort_queries();
        let mut l = 0;
        let mut r = 0;
        for &idx in &ord {
            while l > self.lr[idx].0 {
                l -= 1;
                add(state, l);
            }
            while r < self.lr[idx].1 {
                add(state, r);
                r += 1;
            }
            while l < self.lr[idx].0 {
                erase(state, l);
                l += 1;
            }
            while r > self.lr[idx].1 {
                r -= 1;
                erase(state, r);
            }
            query(state, idx);
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
