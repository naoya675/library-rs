#[derive(Debug, Clone)]
pub struct XorShift128 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShift128 {
    pub fn new(seed: u64) -> Self {
        let mut sm = seed;
        let a = splitmix64(&mut sm);
        let b = splitmix64(&mut sm);
        Self {
            x: a as u32,
            y: (a >> 32) as u32,
            z: b as u32,
            w: (b >> 32) as u32,
        }
    }

    pub fn seed() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    pub fn next_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w
    }

    pub fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | self.next_u32() as u64
    }

    pub fn random_range(&mut self, range: std::ops::Range<u64>) -> u64 {
        assert!(range.start < range.end);
        self.next_u64() % (range.end - range.start) + range.start
    }

    pub fn random_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            let j = self.random_range(0..(i as u64 + 1)) as usize;
            a.swap(i, j);
        }
    }
}

fn splitmix64(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
