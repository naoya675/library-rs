//! https://atcoder.github.io/ac-library/production/document_en/fenwicktree.html

#[derive(Debug, Clone)]
pub struct FenwickTree<T> {
    tree: Vec<T>,
    size: usize,
}

impl<T: Copy + PartialOrd + Ord + Default> FenwickTree<T>
where
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize) -> Self {
        let size = n;
        Self {
            tree: vec![T::default(); size + 1],
            size,
        }
    }

    pub fn add(&mut self, mut k: usize, x: T) {
        assert!(k < self.size);
        k += 1;
        while k <= self.size {
            self.tree[k] += x;
            k += k & k.wrapping_neg();
        }
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.size);
        self.prefix_sum(r) - self.prefix_sum(l)
    }

    fn prefix_sum(&self, mut r: usize) -> T {
        let mut s = T::default();
        while r > 0 {
            s += self.tree[r];
            r -= r & r.wrapping_neg();
        }
        s
    }

    // unverify
    pub fn lower_bound(&self, mut x: T) -> usize {
        let mut s = 0;
        let mut k = self.size.next_power_of_two();
        while k > 0 {
            if s + k <= self.size && self.tree[s + k] < x {
                x -= self.tree[s + k];
                s += k;
            }
            k >>= 1;
        }
        s
    }

    // unverify
    pub fn upper_bound(&self, mut x: T) -> usize {
        let mut s = 0;
        let mut k = self.size.next_power_of_two();
        while k > 0 {
            if s + k <= self.size && self.tree[s + k] <= x {
                x -= self.tree[s + k];
                s += k;
            }
            k >>= 1;
        }
        s
    }
}
