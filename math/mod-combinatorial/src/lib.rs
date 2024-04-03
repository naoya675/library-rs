use mod_int::ModInt;

#[derive(Debug, Clone)]
pub struct ModCombinatorial<const MOD: u64> {
    fact: Vec<ModInt<MOD>>,
    finv: Vec<ModInt<MOD>>,
}

impl<const MOD: u64> ModCombinatorial<MOD> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![ModInt::<MOD>::new(1); n + 1];
        let mut finv = vec![ModInt::<MOD>::new(1); n + 1];
        for i in 0..n {
            fact[i + 1] = fact[i] * ModInt::<MOD>::new((i + 1) as u64);
        }
        finv[n] = fact[n].inv();
        for i in (0..n).rev() {
            finv[i] = finv[i + 1] * ModInt::<MOD>::new((i + 1) as u64);
        }
        Self { fact, finv }
    }

    pub fn fact(&self, n: usize) -> ModInt<MOD> {
        assert!(n <= self.fact.len());
        self.fact[n]
    }

    pub fn finv(&self, n: usize) -> ModInt<MOD> {
        assert!(n <= self.finv.len());
        self.finv[n]
    }

    // permutation
    pub fn perm(&self, n: usize, r: usize) -> ModInt<MOD> {
        // assert!(r <= n);
        if r > n {
            return ModInt::<MOD>::new(0);
        }
        self.fact[n] * self.finv[n - r]
    }

    // combination
    pub fn comb(&self, n: usize, r: usize) -> ModInt<MOD> {
        // assert!(r <= n);
        if r > n {
            return ModInt::<MOD>::new(0);
        }
        self.fact[n] * self.finv[r] * self.finv[n - r]
    }

    // homogeneous product
    pub fn homo(&self, n: usize, r: usize) -> ModInt<MOD> {
        self.comb(n + r - 1, r)
    }
}
