#[derive(Debug, Clone)]
pub struct SegmentTree<T> {
    tree: Vec<T>,
    size: usize,
    op: fn(T, T) -> T, // evaluation funciton
    e: T,              // identity element
}

impl<T: Copy> SegmentTree<T> {
    pub fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        let size = n.next_power_of_two();
        Self {
            tree: vec![e; 2 * size],
            size,
            op,
            e,
        }
    }

    pub fn build(&mut self, v: Vec<T>) {
        for i in 0..v.len() {
            self.set(i, v[i]);
        }
    }

    pub fn set(&mut self, mut k: usize, x: T) {
        assert!(k < self.size);
        k += self.size;
        self.tree[k] = x;
        while k > 0 {
            k /= 2;
            self.update(k);
        }
    }

    pub fn get(&mut self, mut k: usize) -> T {
        assert!(k < self.size);
        k += self.size;
        self.tree[k].clone()
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        if l == r {
            return self.e;
        }
        l += self.size;
        r += self.size;
        let mut l_res = self.e;
        let mut r_res = self.e;
        while l < r {
            if l % 2 == 1 {
                l_res = (self.op)(l_res, self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                r_res = (self.op)(self.tree[r], r_res);
            }
            l /= 2;
            r /= 2;
        }
        (self.op)(l_res, r_res)
    }

    pub fn all_prod(&mut self) -> T {
        self.tree[1].clone()
    }

    pub fn apply(&mut self, mut k: usize, x: T) {
        assert!(k < self.size);
        k += self.size;
        self.tree[k] = (self.op)(self.tree[k], x);
        while k > 0 {
            k /= 2;
            self.tree[k] = (self.op)(self.tree[k << 1 | 0], self.tree[k << 1 | 1]);
        }
    }

    fn update(&mut self, k: usize) {
        self.tree[k] = (self.op)(self.tree[k << 1 | 0], self.tree[k << 1 | 1]);
    }
}
