#[derive(Debug, Clone)]
pub struct SuccinctIndexableDictionary {
    size: usize,
    bits: Vec<u64>,
    level1: Vec<u64>,
    level2: Vec<u16>,
    count_ones: usize,
}

impl SuccinctIndexableDictionary {
    const WORD_SIZE: usize = 64;
    const LEVEL1_SIZE: usize = 512;
    const LEVEL2_SIZE: usize = 64;
    const LEVEL2_PER_LEVEL1: usize = Self::LEVEL1_SIZE / Self::LEVEL2_SIZE; // = 8

    pub fn new(size: usize) -> Self {
        let bits_size = (size + Self::WORD_SIZE - 1) / Self::WORD_SIZE + 1;
        let level1_size = (size + Self::LEVEL1_SIZE - 1) / Self::LEVEL1_SIZE + 1;
        let level2_size = (size + Self::LEVEL2_SIZE - 1) / Self::LEVEL2_SIZE + 1;
        Self {
            size,
            bits: vec![0; bits_size],
            level1: vec![0; level1_size],
            level2: vec![0; level2_size],
            count_ones: 0,
        }
    }

    pub fn set(&mut self, k: usize) {
        assert!(k < self.size);
        self.bits[k / Self::WORD_SIZE] |= 1u64 << (k % Self::WORD_SIZE);
    }

    /*
    pub fn build(&mut self) {
        let mut count = 0;
        for i in 0..=self.size {
            if i % Self::LEVEL1_SIZE == 0 {
                self.level1[i / Self::LEVEL1_SIZE] = count;
            }
            if i % Self::LEVEL2_SIZE == 0 {
                self.level2[i / Self::LEVEL2_SIZE] = (count - self.level1[i / Self::LEVEL1_SIZE]) as u16;
            }
            if i < self.size && i % Self::LEVEL2_SIZE == 0 {
                count += Self::pop_count(self.bits[i / Self::LEVEL2_SIZE]);
            }
        }
        self.count_ones = count as usize;
    }
     */

    pub fn build(&mut self) {
        let mut count = 0;
        for (i, &bit) in self.bits.iter().enumerate() {
            if i % Self::LEVEL2_PER_LEVEL1 == 0 {
                self.level1[i / Self::LEVEL2_PER_LEVEL1] = count;
            }
            self.level2[i] = (count - self.level1[i / Self::LEVEL2_PER_LEVEL1]) as u16;
            count += Self::pop_count(bit);
        }
        self.count_ones = count as usize;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn access(&self, k: usize) -> bool {
        assert!(k < self.size);
        (self.bits[k / Self::WORD_SIZE] >> (k % Self::WORD_SIZE)) & 1 == 1
    }

    pub fn rank1(&self, k: usize) -> usize {
        assert!(k <= self.size);
        let level1_id = k / Self::LEVEL1_SIZE;
        let level2_id = k / Self::LEVEL2_SIZE;
        let offset = k % Self::LEVEL2_SIZE;
        let mask = (1u64 << offset).wrapping_sub(1);
        self.level1[level1_id] as usize + self.level2[level2_id] as usize + Self::pop_count(self.bits[level2_id] & mask) as usize
    }

    pub fn rank0(&self, k: usize) -> usize {
        assert!(k <= self.size);
        k - self.rank1(k)
    }

    pub fn rank(&self, bit: bool, k: usize) -> usize {
        if bit { self.rank1(k) } else { self.rank0(k) }
    }

    pub fn select1(&self, k: usize) -> usize {
        assert!(k < self.count_ones);
        let mut hi = self.size;
        let mut lo = 0;
        while lo < hi {
            let mi = (lo + hi) / 2;
            if self.rank1(mi + 1) <= k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        lo
    }

    pub fn select0(&self, k: usize) -> usize {
        assert!(k < self.size - self.count_ones);
        let mut hi = self.size;
        let mut lo = 0;
        while lo < hi {
            let mi = (lo + hi) / 2;
            if self.rank0(mi + 1) <= k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        lo
    }

    pub fn select(&self, bit: bool, k: usize) -> usize {
        if bit { self.select1(k) } else { self.select0(k) }
    }

    fn pop_count(mut x: u64) -> u64 {
        x = (x & 0x5555_5555_5555_5555) + ((x >> 1) & 0x5555_5555_5555_5555);
        x = (x & 0x3333_3333_3333_3333) + ((x >> 2) & 0x3333_3333_3333_3333);
        x = (x + (x >> 4)) & 0x0f0f_0f0f_0f0f_0f0f;
        x = x + (x >> 8);
        x = x + (x >> 16);
        x = x + (x >> 32);
        x & 0x7f
    }
}
