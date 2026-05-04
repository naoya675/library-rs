#[derive(Debug, Clone)]
pub struct LevelAncestor {
    n: usize,
    graph: Vec<Vec<usize>>,
    depth: Vec<usize>,
    count: Vec<usize>,
    intime: Vec<usize>,
    intime_inv: Vec<usize>,
    bucket: Vec<usize>,
}

impl LevelAncestor {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![vec![]; n],
            depth: vec![0; n],
            count: vec![0; n + 1],
            intime: vec![0; n],
            intime_inv: vec![0; n],
            bucket: vec![0; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.n);
        assert!(v < self.n);
        self.graph[u].push(v);
    }

    pub fn init(&mut self, root: usize) {
        assert!(root < self.n);
        let mut time = 0;
        let mut stack = vec![(root, None)];
        while let Some((v, par)) = stack.pop() {
            self.intime[v] = time;
            self.intime_inv[time] = v;
            time += 1;
            self.count[self.depth[v] + 1] += 1;
            for &to in &self.graph[v] {
                if Some(to) != par {
                    self.depth[to] = self.depth[v] + 1;
                    stack.push((to, Some(v)));
                }
            }
        }
        for i in 0..self.n {
            self.count[i + 1] += self.count[i];
        }
        let mut idx = self.count.clone();
        for &v in &self.intime_inv {
            self.bucket[idx[self.depth[v]]] = self.intime[v];
            idx[self.depth[v]] += 1;
        }
    }

    pub fn la(&self, v: usize, k: usize) -> Option<usize> {
        assert!(v < self.n);
        if k > self.depth[v] {
            return None;
        }
        let d = self.depth[v] - k;
        let lo = self.count[d];
        let hi = self.count[d + 1];
        let i = self.bucket[lo..hi].partition_point(|&t| t <= self.intime[v]);
        Some(self.intime_inv[self.bucket[lo..hi][i - 1]])
    }
}
