#[derive(Debug, Clone)]
pub struct PartiallyPersistentUnionFind {
    n: usize,
    now: usize,
    par: Vec<usize>,
    par_time: Vec<Option<usize>>,
    siz_history: Vec<Vec<(usize, usize)>>,
}

impl PartiallyPersistentUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            now: 0,
            par: (0..n).collect(),
            par_time: vec![None; n],
            siz_history: vec![vec![(0, 1)]; n],
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
        self.now += 1;
        let mut x = self.leader(self.now, x);
        let mut y = self.leader(self.now, y);
        if x == y {
            return x;
        }
        let sx = self.siz_history[x].last().unwrap().1;
        let sy = self.siz_history[y].last().unwrap().1;
        if sx < sy {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[y] = x;
        self.par_time[y] = Some(self.now);
        self.siz_history[x].push((self.now, sx + sy));
        x
    }

    pub fn same(&self, t: usize, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        self.leader(t, x) == self.leader(t, y)
    }

    pub fn leader(&self, t: usize, x: usize) -> usize {
        assert!(x < self.n);
        if self.par_time[x].is_some_and(|tt| tt <= t) {
            self.leader(t, self.par[x])
        } else {
            x
        }
    }

    pub fn size(&self, t: usize, x: usize) -> usize {
        assert!(x < self.n);
        let r = self.leader(t, x);
        let k = self.siz_history[r].partition_point(|&(tt, _)| tt <= t);
        self.siz_history[r][k - 1].1
    }

    pub fn now(&self) -> usize {
        self.now
    }
}
