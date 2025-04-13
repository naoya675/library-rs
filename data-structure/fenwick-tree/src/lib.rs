#[derive(Debug, Clone)]
pub struct FenwickTree<T> {
    tree: Vec<T>,
    size: usize,
}

impl<T: Copy> FenwickTree<T>
where
    T: std::ops::Neg<Output = T>,
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
    T: num_traits::Zero,
{
    pub fn new(n: usize) -> Self {
        let size = n;
        Self {
            tree: vec![T::zero(); size],
            size,
        }
    }

    pub fn build(&mut self, v: Vec<T>) {
        assert!(v.len() <= self.size);
        for i in 0..v.len() {
            self.add(i, v[i]);
        }
    }

    pub fn add(&mut self, mut k: usize, x: T) {
        assert!(k < self.size);
        k += 1;
        while k <= self.size {
            self.tree[k - 1] += x;
            k += k & k.wrapping_neg();
        }
    }

    pub fn sum(&mut self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.size);
        self.prefix_sum(r) - self.prefix_sum(l)
    }

    fn prefix_sum(&mut self, mut r: usize) -> T {
        let mut s = T::zero();
        while r > 0 {
            s += self.tree[r - 1];
            r -= r & r.wrapping_neg();
        }
        s
    }
}
