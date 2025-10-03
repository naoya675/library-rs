// Reference: https://atcoder.github.io/ac-library/production/document_en/fenwicktree.html

#[derive(Debug, Clone)]
pub struct FenwickTreeAbstract<T> {
    tree: Vec<T>,
    size: usize,
    // Abelian Group: operation (associative, commutative) + identity element + inverse element
    op: fn(T, T) -> T,
    e: T,
    inv: fn(T) -> T,
}

impl<T: Copy> FenwickTreeAbstract<T>
where
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize, op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self {
        let size = n;
        Self {
            tree: vec![e; size + 1],
            size,
            op,
            e,
            inv,
        }
    }

    // apply
    pub fn add(&mut self, mut k: usize, x: T) {
        assert!(k < self.size);
        k += 1;
        while k <= self.size {
            self.tree[k] = (self.op)(self.tree[k], x);
            k += k & k.wrapping_neg();
        }
    }

    // prod
    pub fn sum(&mut self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.size);
        (self.op)(self.prefix_sum(r), (self.inv)(self.prefix_sum(l)))
    }

    pub fn prefix_sum(&mut self, mut r: usize) -> T {
        let mut s = self.e;
        while r > 0 {
            s = (self.op)(s, self.tree[r]);
            r -= r & r.wrapping_neg();
        }
        s
    }
}
