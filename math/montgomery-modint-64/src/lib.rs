#[derive(Clone, Copy)]
pub struct MontgomeryModint64<const M: u64> {
    value: u64, // Montgomery form (a * R mod M), R = 2^64
}

impl<const M: u64> PartialEq for MontgomeryModint64<M> {
    fn eq(&self, other: &Self) -> bool {
        let a = if self.value >= M { self.value - M } else { self.value };
        let b = if other.value >= M { other.value - M } else { other.value };
        a == b
    }
}

impl<const M: u64> Eq for MontgomeryModint64<M> {}

impl<const M: u64> MontgomeryModint64<M> {
    const INV_M: u64 = Self::get_inv_m(M);
    const R2: u64 = Self::get_r2(M);

    // INV_M * M ≡ 1 (mod 2^64)
    const fn get_inv_m(m: u64) -> u64 {
        let mut r = m;
        let mut i = 0;
        while i < 5 {
            r = r.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(r)));
            i += 1;
        }
        r
    }

    // R2 = R^2 mod M = 2^128 mod M
    const fn get_r2(m: u64) -> u64 {
        let mut r = 1u64 % m;
        let mut i = 0;
        while i < 128 {
            r <<= 1;
            if r >= m {
                r -= m;
            }
            i += 1;
        }
        r
    }

    fn mul_high(x: u64, y: u64) -> u64 {
        let mask = (1u64 << 32) - 1;
        let xh = x >> 32;
        let yh = y >> 32;
        let xl = x & mask;
        let yl = y & mask;
        let mid = (xh * yl & mask) + (xl * yh & mask) + (xl * yl >> 32);
        xh * yh + (xh * yl >> 32) + (xl * yh >> 32) + (mid >> 32)
    }

    fn mul_low(x: u64, y: u64) -> u64 {
        x.wrapping_mul(y)
    }

    // Multiplication + Montgomery reduction: x * y * R^{-1} mod M
    fn mul_reduce(x: u64, y: u64) -> u64 {
        let hi = Self::mul_high(x, y);
        let lo = Self::mul_low(x, y);
        let correction = Self::mul_high(Self::mul_low(lo, Self::INV_M.wrapping_neg()), M);
        let carry = if lo == 0 { 0 } else { 1 };
        hi + correction + carry
    }

    pub fn new(n: i64) -> Self {
        const { assert!(M & 1 == 1) };
        let v = n.rem_euclid(M as i64) as u64;
        Self {
            value: Self::mul_reduce(v, Self::R2),
        }
    }

    pub fn value(&self) -> u64 {
        let res = Self::mul_reduce(self.value, 1);
        if res >= M { res - M } else { res }
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
        let (x, _, _) = Self::ext_gcd(self.value() as i64, M as i64);
        Self::new(x as i64)
    }
}

impl<const M: u64> std::ops::Add for MontgomeryModint64<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut value = self.value + rhs.value;
        if value >= 2 * M {
            value -= 2 * M;
        }
        Self { value }
    }
}

impl<const M: u64> std::ops::AddAssign for MontgomeryModint64<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const M: u64> std::ops::Sub for MontgomeryModint64<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut value = 2 * M + self.value - rhs.value;
        if value >= 2 * M {
            value -= 2 * M;
        }
        Self { value }
    }
}

impl<const M: u64> std::ops::SubAssign for MontgomeryModint64<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const M: u64> std::ops::Mul for MontgomeryModint64<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: Self::mul_reduce(self.value, rhs.value),
        }
    }
}

impl<const M: u64> std::ops::MulAssign for MontgomeryModint64<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const M: u64> std::ops::Div for MontgomeryModint64<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(rhs.value() != 0);
        self * rhs.inv()
    }
}

impl<const M: u64> std::ops::DivAssign for MontgomeryModint64<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const M: u64> std::ops::Neg for MontgomeryModint64<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

use numeric_traits::{Numeric, One, Zero};

impl<const M: u64> Zero for MontgomeryModint64<M> {
    fn zero() -> Self {
        Self::new(0)
    }
}

impl<const M: u64> One for MontgomeryModint64<M> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<const M: u64> Numeric for MontgomeryModint64<M> {}

impl<const M: u64> Default for MontgomeryModint64<M> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<const M: u64> std::fmt::Debug for MontgomeryModint64<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<const M: u64> std::fmt::Display for MontgomeryModint64<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl<const M: u64> From<$type> for MontgomeryModint64<M> {
                fn from(value: $type) -> Self {
                    Self::new(value as i64)
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
