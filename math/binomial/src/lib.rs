#[derive(Debug, Clone)]
pub struct Binomial<T> {
    fact: Vec<T>,
    finv: Vec<T>,
    inv: Vec<T>,
}

impl<T: Copy + From<u64>> Binomial<T>
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
    pub fn new() -> Self {
        Self {
            fact: vec![T::from(1u64)], // fact[0] = 1
            finv: vec![T::from(1u64)], // finv[0] = 1
            inv: vec![T::from(0u64)],  // inv[0] is undefined; placeholder
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        let mut bi = Self::new();
        bi.extend(n);
        bi
    }

    fn extend(&mut self, n: usize) {
        let m = self.fact.len();
        if n < m {
            return;
        }
        self.fact.resize(n + 1, T::from(1u64));
        self.finv.resize(n + 1, T::from(1u64));
        self.inv.resize(n + 1, T::from(0u64));
        for i in m..=n {
            self.fact[i] = self.fact[i - 1] * T::from(i as u64);
        }
        self.finv[n] = T::from(1u64) / self.fact[n];
        for i in (m..n).rev() {
            self.finv[i] = self.finv[i + 1] * T::from((i + 1) as u64);
        }
        for i in m..=n {
            self.inv[i] = self.finv[i] * self.fact[i - 1];
        }
    }

    pub fn fact(&mut self, n: usize) -> T {
        self.extend(n);
        self.fact[n]
    }

    pub fn finv(&mut self, n: usize) -> T {
        self.extend(n);
        self.finv[n]
    }

    pub fn inv(&mut self, n: usize) -> T {
        assert!(n > 0, "inv(0) is undefined");
        self.extend(n);
        self.inv[n]
    }

    // permutation
    pub fn perm(&mut self, n: usize, r: usize) -> T {
        if n < r {
            return T::from(0u64);
        }
        self.fact(n) * self.finv(n - r)
    }

    // combination
    pub fn comb(&mut self, n: usize, r: usize) -> T {
        if n < r {
            return T::from(0u64);
        }
        self.fact(n) * self.finv(r) * self.finv(n - r)
    }

    // multinomial coefficient
    pub fn multinomial(&mut self, k: &[usize]) -> T {
        let mut res = self.fact(k.iter().sum());
        for &k in k {
            res *= self.finv(k);
        }
        res
    }

    // combinations with replacement (homogeneous product)
    pub fn homo(&mut self, n: usize, r: usize) -> T {
        if n == 0 {
            return if r == 0 { T::from(1u64) } else { T::from(0u64) };
        }
        self.comb(n + r - 1, r)
    }

    // catalan number
    pub fn catalan(&mut self, n: usize) -> T {
        // self.comb(2 * n, n) - self.comb(2 * n, n - 1)
        self.comb(2 * n, n) / T::from((n + 1) as u64)
    }
}

impl<T: Copy + From<u64>> Default for Binomial<T>
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
    fn default() -> Self {
        Self::new()
    }
}
