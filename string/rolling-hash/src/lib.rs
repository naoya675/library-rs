#[derive(Debug, Clone)]
pub struct RollingHash<T> {
    base: T,
    power: Vec<T>,
}

impl<T: Copy + From<u64>> RollingHash<T>
where
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
    T: std::ops::Mul<T, Output = T>,
    T: std::ops::MulAssign,
    T: std::ops::Div<T, Output = T>,
    T: std::ops::DivAssign,
{
    pub fn new(base: T) -> Self {
        Self {
            base,
            power: vec![T::from(1u64)],
        }
    }

    pub fn build(&mut self, s: &Vec<char>) -> Vec<T> {
        let size = s.len();
        let mut hash = vec![T::from(0u64); size + 1];
        for i in 0..size {
            hash[i + 1] = hash[i] * self.base + T::from(s[i] as u64);
        }
        hash
    }

    fn build_power(&mut self, r: usize) {
        while self.power.len() <= r {
            let val = *self.power.last().unwrap();
            self.power.push(val * self.base);
        }
    }

    // [l, r)
    pub fn rolling_hash(&mut self, hash: &Vec<T>, l: usize, r: usize) -> T {
        assert!(l <= r && r <= hash.len());
        self.build_power(r - l);
        hash[r] - hash[l] * self.power[r - l]
    }
}
