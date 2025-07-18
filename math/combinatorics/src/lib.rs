#[derive(Debug, Clone)]
pub struct Combinatorics<T> {
    fact: Vec<T>,
    finv: Vec<T>,
    inv: Vec<T>,
}

impl<T: Copy + From<u64>> Combinatorics<T>
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
    pub fn new(n: usize) -> Self {
        let mut fact = vec![T::from(1u64); n + 1];
        let mut finv = vec![T::from(1u64); n + 1];
        let mut inv = vec![T::from(1u64); n + 1];
        for i in 0..n {
            fact[i + 1] = fact[i] * T::from((i + 1) as u64);
        }
        finv[n] = T::from(1u64) / fact[n];
        for i in (0..n).rev() {
            finv[i] = finv[i + 1] * T::from((i + 1) as u64);
        }
        for i in 0..n {
            inv[i + 1] = finv[i + 1] * fact[i];
        }
        Self { fact, finv, inv }
    }

    pub fn fact(&mut self, n: usize) -> T {
        assert!(n <= self.fact.len());
        self.fact[n]
    }

    pub fn finv(&mut self, n: usize) -> T {
        assert!(n <= self.finv.len());
        self.finv[n]
    }

    pub fn inv(&mut self, n: usize) -> T {
        assert!(n <= self.inv.len());
        self.inv[n]
    }

    // permutation
    pub fn perm(&mut self, n: usize, r: usize) -> T {
        // assert!(r <= n);
        if n < r {
            return T::from(0u64);
        }
        self.fact(n) * self.finv(n - r)
    }

    // combination
    pub fn comb(&mut self, n: usize, r: usize) -> T {
        // assert!(r <= n);
        if n < r {
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
