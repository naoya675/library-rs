#[derive(Debug, Clone)]
pub struct MoWithRollback {
    n: usize,
    w: usize,
    lr: Vec<(usize, usize)>,
}

impl MoWithRollback {
    pub fn new(n: usize, q: usize) -> Self {
        let w = std::cmp::max(1, n / std::cmp::max(1, (q as f64).sqrt() as usize));
        Self { n, w, lr: Vec::new() }
    }

    pub fn add_query(&mut self, l: usize, r: usize) {
        assert!(l < r && r <= self.n);
        self.lr.push((l, r));
    }

    pub fn run_queries<A, R, SN, RB, Q>(&self, mut add: A, mut reset: R, mut snapshot: SN, mut rollback: RB, mut query: Q)
    where
        A: FnMut(usize),
        R: FnMut(),
        SN: FnMut(),
        RB: FnMut(),
        Q: FnMut(usize),
    {
        let w = self.w;
        let ord = self.sort_queries();
        reset();
        for &idx in &ord {
            let (l, r) = self.lr[idx];
            if r - l < w {
                snapshot();
                for i in l..r {
                    add(i);
                }
                query(idx);
                rollback();
            }
        }
        let mut nr = 0;
        let mut last_block = None;
        for &idx in &ord {
            let (l, r) = self.lr[idx];
            if r - l < w {
                continue;
            }
            let block = l / w;
            if last_block != Some(block) {
                reset();
                last_block = Some(block);
                nr = (block + 1) * w;
            }
            while nr < r {
                add(nr);
                nr += 1;
            }
            snapshot();
            for j in (l..(block + 1) * w).rev() {
                add(j);
            }
            query(idx);
            rollback();
        }
    }

    fn sort_queries(&self) -> Vec<usize> {
        let q = self.lr.len();
        let mut ord: Vec<usize> = (0..q).collect();
        let w = self.w;
        ord.sort_by_key(|&i| (self.lr[i].0 / w, self.lr[i].1));
        ord
    }
}
