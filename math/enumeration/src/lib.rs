#[derive(Debug, Clone)]
pub struct Enumeration<T> {
    fact: Vec<T>,
    finv: Vec<T>,
}

impl<T: Copy + From<u64>> Enumeration<T>
where
    T: std::ops::Neg<Output = T>,
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
    T: std::ops::Mul<T, Output = T>,
    T: std::ops::MulAssign,
    T: std::ops::Div<T, Output = T>,
    T: std::ops::DivAssign,
{
    pub fn new(n: usize) -> Self {
        let mut fact = vec![T::from(1u64); n + 1];
        let mut finv = vec![T::from(1u64); n + 1];
        for i in 0..n {
            fact[i + 1] = fact[i] * T::from((i + 1) as u64);
        }
        finv[n] = T::from(1u64) / fact[n];
        for i in (0..n).rev() {
            finv[i] = finv[i + 1] * T::from((i + 1) as u64);
        }
        Self { fact, finv }
    }

    fn update(&mut self, n: usize) {
        if self.fact.len() < n + 1 {
            let len = self.fact.len();
            self.fact.resize(n + 1, T::from(1u64));
            self.finv.resize(n + 1, T::from(1u64));
            for i in len - 1..n {
                self.fact[i + 1] = self.fact[i] * T::from((i + 1) as u64);
            }
            self.finv[n] = T::from(1u64) / self.fact[n];
            for i in (len - 1..n).rev() {
                self.finv[i] = self.finv[i + 1] * T::from((i + 1) as u64);
            }
        }
    }

    pub fn fact(&mut self, n: usize) -> T {
        // assert!(n <= self.fact.len());
        self.update(n);
        self.fact[n]
    }

    pub fn finv(&mut self, n: usize) -> T {
        // assert!(n <= self.finv.len());
        self.update(n);
        self.finv[n]
    }

    // permutation
    pub fn perm(&mut self, n: usize, r: usize) -> T {
        // assert!(r <= n);
        if r > n {
            return T::from(0u64);
        }
        self.fact(n) * self.finv(n - r)
    }

    // combination
    pub fn comb(&mut self, n: usize, r: usize) -> T {
        // assert!(r <= n);
        if r > n {
            return T::from(0u64);
        }
        self.fact(n) * self.finv(r) * self.finv(n - r)
    }

    // combinations with replacement (homogeneous product)
    pub fn homo(&mut self, n: usize, r: usize) -> T {
        self.comb(n + r - 1, r)
    }

    // catalan number
    pub fn catalan(&mut self, n: usize) -> T {
        self.comb(2 * n, n) - self.comb(2 * n, n - 1)
    }
}
