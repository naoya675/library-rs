use fenwick_tree_abstract::FenwickTreeAbstract;

#[derive(Debug, Clone)]
pub struct FenwickTreeRangeAdd {
    tree: FenwickTreeAbstract<(i64, i64)>,
    size: usize,
}

impl FenwickTreeRangeAdd {
    pub fn new(n: usize) -> Self {
        Self {
            tree: FenwickTreeAbstract::new(n + 1, |x, y| (x.0 + y.0, x.1 + y.1), (0, 0), |x| (-x.0, -x.1)),
            size: n,
        }
    }

    pub fn from_slice(v: &[i64]) -> Self {
        assert!(v.len() > 0);
        let n = v.len();
        let mut tree = vec![(0, 0); n + 1];
        for i in 0..n {
            tree[i].0 = v[i];
        }
        Self {
            tree: FenwickTreeAbstract::from_slice(&tree, |x, y| (x.0 + y.0, x.1 + y.1), (0, 0), |x| (-x.0, -x.1)),
            size: n,
        }
    }

    pub fn add(&mut self, l: usize, r: usize, x: i64) {
        assert!(l <= r && r <= self.size);
        self.tree.add(l, (-x * l as i64, x));
        self.tree.add(r, (x * r as i64, -x));
    }

    pub fn sum(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r <= self.size);
        self.prefix_sum(r) - self.prefix_sum(l)
    }

    fn prefix_sum(&self, r: usize) -> i64 {
        let (sum0, sum1) = self.tree.sum(0, r);
        sum0 + sum1 * r as i64
    }
}
