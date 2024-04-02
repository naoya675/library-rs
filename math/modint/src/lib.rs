#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModInt<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> ModInt<MOD> {
    pub fn new(n: u64) -> Self {
        Self {
            value: (n % MOD),
            // value: (n.rem_euclid(MOD)),
        }
    }

    pub fn value(&self) -> u64 {
        self.value % MOD
    }

    pub fn power(&self, mut n: u64) -> Self {
        let mut value = self.value;
        let mut res = 1;
        while n > 0 {
            if n & 1 != 0 {
                res = (res * value) % MOD;
            }
            value = (value * value) % MOD;
            n >>= 1;
        }
        Self { value: res }
    }

    pub fn pow(&self, n: Self) -> Self {
        self.power(n.value)
    }

    pub fn inv(&self) -> Self {
        self.power(MOD - 2)
    }
}

impl<const MOD: u64> std::ops::Add for ModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: (self.value + rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            value: (self.value + rhs.value) % MOD,
        };
    }
}

impl<const MOD: u64> std::ops::Sub for ModInt<MOD> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        if self.value < rhs.value {
            self.value += MOD;
        }
        Self {
            value: (self.value - rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        *self = Self {
            value: (self.value - rhs.value) % MOD,
        };
    }
}

impl<const MOD: u64> std::ops::Mul for ModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: (self.value * rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            value: (self.value * rhs.value) % MOD,
        };
    }
}

impl<const MOD: u64> std::ops::Div for ModInt<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.value == 0 {
            panic!();
        }
        self * rhs.inv()
    }
}

impl<const MOD: u64> std::ops::DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.value == 0 {
            panic!();
        }
        *self *= rhs.inv();
    }
}

impl<const MOD: u64> std::fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
