#[derive(Debug, Clone)]
pub struct FenwickTreeAbstract<T> {
    tree: Vec<T>,
    size: usize,
    // Abelian Group: operation (associative, commutative) + identity element + inverse element
    op: fn(T, T) -> T,
    e: T,
    inv: fn(T) -> T,
}

impl<T: Copy> FenwickTreeAbstract<T> {
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

    pub fn add(&mut self, mut k: usize, x: T) {
        assert!(k < self.size);
        k += 1;
        while k <= self.size {
            self.tree[k] = (self.op)(self.tree[k], x);
            k += k & k.wrapping_neg();
        }
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.size);
        (self.op)(self.prefix_sum(r), (self.inv)(self.prefix_sum(l)))
    }

    fn prefix_sum(&self, mut r: usize) -> T {
        let mut s = self.e;
        while r > 0 {
            s = (self.op)(s, self.tree[r]);
            r -= r & r.wrapping_neg();
        }
        s
    }
}
