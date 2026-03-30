#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticModint<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> StaticModint<MOD> {
    pub fn new(n: i64) -> Self {
        Self {
            value: n.rem_euclid(MOD as i64) as u64,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    // ax + by = gcd(a, b) -> (x mod b, y mod b, gcd(a, b))
    fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        let (mut x0, mut y0, mut r0) = (1, 0, a);
        let (mut x1, mut y1, mut r1) = (0, 1, b);

        while r1 != 0 {
            let t = r0 / r1;
            x0 -= t * x1;
            y0 -= t * y1;
            r0 -= t * r1;

            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
            std::mem::swap(&mut r0, &mut r1);
        }
        (x0.rem_euclid(b), y0.rem_euclid(b), r0.rem_euclid(b))
    }

    pub fn pow(&self, mut n: u64) -> Self {
        let mut value = *self;
        let mut res = Self::new(1);
        while n > 0 {
            if n & 1 != 0 {
                res = res * value;
            }
            value = value * value;
            n >>= 1;
        }
        res
    }

    pub fn inv(&self) -> Self {
        let (x, _, _) = Self::ext_gcd(self.value() as i64, MOD as i64);
        Self { value: x as u64 }
    }
}

impl<const MOD: u64> std::ops::Add for StaticModint<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut value = self.value + rhs.value;
        if value >= MOD {
            value -= MOD;
        }
        Self { value }
    }
}

impl<const MOD: u64> std::ops::AddAssign for StaticModint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: u64> std::ops::Sub for StaticModint<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut value = MOD + self.value - rhs.value;
        if value >= MOD {
            value -= MOD;
        }
        Self { value }
    }
}

impl<const MOD: u64> std::ops::SubAssign for StaticModint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: u64> std::ops::Mul for StaticModint<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: (self.value * rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::MulAssign for StaticModint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: u64> std::ops::Div for StaticModint<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.value == 0 {
            panic!();
        }
        self * rhs.inv()
    }
}

impl<const MOD: u64> std::ops::DivAssign for StaticModint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: u64> std::ops::Neg for StaticModint<MOD> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

use numeric_traits::{Numeric, One, Zero};

impl<const MOD: u64> Zero for StaticModint<MOD> {
    fn zero() -> Self {
        Self::new(0)
    }
}

impl<const MOD: u64> One for StaticModint<MOD> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<const MOD: u64> Numeric for StaticModint<MOD> {}

impl<const MOD: u64> Default for StaticModint<MOD> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<const MOD: u64> std::fmt::Display for StaticModint<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl<const MOD: u64> From<$type> for StaticModint<MOD> {
                fn from(value: $type) -> Self {
                    Self::new(value as i64)
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

/*
macro_rules! impl_ops {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl<const MOD: u64> std::ops::$trait for StaticModint<MOD> {
            fn $fn(&mut self, rhs: Self) {
                *self = *self $op rhs;
            }
        }
    };
}

impl_ops!(AddAssign, add_assign, +);
impl_ops!(SubAssign, sub_assign, -);
impl_ops!(MulAssign, mul_assign, *);
impl_ops!(DivAssign, div_assign, /);
*/
