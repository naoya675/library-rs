use succinct_indexable_dictionary::SuccinctIndexableDictionary;

#[derive(Debug, Clone)]
pub struct WaveletMatrix {
    size: usize,
    size_log: usize,
    matrix: Vec<SuccinctIndexableDictionary>,
    zero_count: Vec<usize>,
}

impl WaveletMatrix {
    pub fn new(v: &[u64]) -> Self {
        assert!(v.len() > 0);
        let max = *v.iter().max().unwrap();
        let size = v.len();
        let size_log = if max == 0 { 1 } else { max.ilog2() as usize + 1 };

        let mut matrix = vec![SuccinctIndexableDictionary::new(size); size_log];
        let mut zero_count = vec![0; size_log];
        let mut cur = v.to_vec();
        for (i, level) in (0..size_log).rev().enumerate() {
            for (j, &x) in cur.iter().enumerate() {
                if (x >> level) & 1 == 1 {
                    matrix[i].set(j);
                }
            }
            matrix[i].build();
            zero_count[i] = matrix[i].rank0(size);
            let mut nxt = vec![];
            for &x in &cur {
                if (x >> level) & 1 == 0 {
                    nxt.push(x);
                }
            }
            for &x in &cur {
                if (x >> level) & 1 == 1 {
                    nxt.push(x);
                }
            }
            cur = nxt;
        }

        Self {
            size,
            size_log,
            matrix,
            zero_count,
        }
    }

    pub fn access(&self, mut k: usize) -> u64 {
        assert!(k < self.size);
        let mut c = 0;
        for i in 0..self.size_log {
            let bit = self.matrix[i].access(k);
            c = (c << 1) | bit as u64;
            k = self.succ(bit, i, k);
        }
        c
    }

    pub fn rank(&self, x: u64, k: usize) -> usize {
        assert!(x < 1 << self.size_log);
        assert!(k <= self.size);
        let mut hi = k;
        let mut lo = 0;
        for i in 0..self.size_log {
            let bit = (x >> (self.size_log - 1 - i)) & 1 == 1;
            hi = self.succ(bit, i, hi);
            lo = self.succ(bit, i, lo);
        }
        hi - lo
    }

    pub fn select(&self, x: u64, k: usize) -> usize {
        assert!(x < 1 << self.size_log);
        let mut hi = self.size;
        let mut lo = 0;
        for i in 0..self.size_log {
            let bit = (x >> (self.size_log - 1 - i)) & 1 == 1;
            hi = self.succ(bit, i, hi);
            lo = self.succ(bit, i, lo);
        }
        assert!(k < hi - lo);
        let mut id = lo + k;
        for i in (0..self.size_log).rev() {
            let bit = (x >> (self.size_log - 1 - i)) & 1 == 1;
            id = self.pred(bit, i, id);
        }
        id
    }

    // [l, r)
    pub fn kth_smallest(&self, l: usize, r: usize, mut k: usize) -> u64 {
        assert!(l <= r && r <= self.size);
        assert!(k < r - l);
        let mut hi = r;
        let mut lo = l;
        let mut ret = 0;
        for i in 0..self.size_log {
            let lo0 = self.succ(false, i, lo);
            let hi0 = self.succ(false, i, hi);
            let zeros = hi0 - lo0;
            if k < zeros {
                ret <<= 1;
                hi = hi0;
                lo = lo0;
            } else {
                k -= zeros;
                ret = (ret << 1) | 1;
                hi = self.succ(true, i, hi);
                lo = self.succ(true, i, lo);
            }
        }
        ret
    }

    fn succ(&self, bit: bool, i: usize, k: usize) -> usize {
        self.matrix[i].rank(bit, k) + if bit { self.zero_count[i] } else { 0 }
    }

    fn pred(&self, bit: bool, i: usize, k: usize) -> usize {
        self.matrix[i].select(bit, k - if bit { self.zero_count[i] } else { 0 })
    }
}
