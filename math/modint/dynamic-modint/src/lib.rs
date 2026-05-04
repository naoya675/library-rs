use std::sync::atomic::{AtomicU64, Ordering};

pub struct BarrettReduction {
    modulus: AtomicU64,
    im: AtomicU64, // floor(2^64 / m)
}

impl BarrettReduction {
    pub const fn new(m: u64) -> Self {
        Self {
            modulus: AtomicU64::new(m),
            im: AtomicU64::new(u64::MAX / m),
        }
    }

    pub fn set(&self, m: u64) {
        self.modulus.store(m, Ordering::Relaxed);
        self.im.store(u64::MAX / m, Ordering::Relaxed);
    }

    pub fn modulus(&self) -> u64 {
        self.modulus.load(Ordering::Relaxed)
    }

    fn im(&self) -> u64 {
        self.im.load(Ordering::Relaxed)
    }

    // Barrett reduction: lhs * rhs mod m (lhs, rhs < m < 2^31)
    pub fn mul(&self, lhs: u64, rhs: u64) -> u64 {
        let m = self.modulus();
        let z = lhs * rhs;
        let x = ((z as u128 * self.im() as u128) >> 64) as u64;
        let v = z - x * m;
        if v >= m { v - m } else { v }
    }
}

pub trait Id {
    fn barrett() -> &'static BarrettReduction;
}

pub struct DefaultId;

impl Id for DefaultId {
    fn barrett() -> &'static BarrettReduction {
        static BR: BarrettReduction = BarrettReduction::new(998244353);
        &BR
    }
}

pub struct DynamicModint<I: Id> {
    value: u64,
    _phantom: std::marker::PhantomData<fn() -> I>,
}

impl<I: Id> Clone for DynamicModint<I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: Id> Copy for DynamicModint<I> {}

impl<I: Id> PartialEq for DynamicModint<I> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<I: Id> Eq for DynamicModint<I> {}

impl<I: Id> DynamicModint<I> {
    pub fn set_mod(m: u64) {
        I::barrett().set(m);
    }

    pub fn get_mod() -> u64 {
        I::barrett().modulus()
    }

    pub fn new(n: i64) -> Self {
        Self {
            value: n.rem_euclid(Self::get_mod() as i64) as u64,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
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
        let (x, _, _) = Self::ext_gcd(self.value() as i64, Self::get_mod() as i64);
        Self {
            value: x as u64,
            _phantom: std::marker::PhantomData,
        }
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
}

impl<I: Id> std::ops::Add for DynamicModint<I> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut value = self.value + rhs.value;
        if value >= Self::get_mod() {
            value -= Self::get_mod();
        }
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> std::ops::AddAssign for DynamicModint<I> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<I: Id> std::ops::Sub for DynamicModint<I> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut value = Self::get_mod() + self.value - rhs.value;
        if value >= Self::get_mod() {
            value -= Self::get_mod();
        }
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> std::ops::SubAssign for DynamicModint<I> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<I: Id> std::ops::Mul for DynamicModint<I> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            // value: (self.value * rhs.value) % Self::get_mod(),
            value: I::barrett().mul(self.value, rhs.value),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> std::ops::MulAssign for DynamicModint<I> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<I: Id> std::ops::Div for DynamicModint<I> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(rhs.value != 0);
        self * rhs.inv()
    }
}

impl<I: Id> std::ops::DivAssign for DynamicModint<I> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<I: Id> std::ops::Neg for DynamicModint<I> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

use numeric_traits::{Numeric, One, Zero};

impl<I: Id> Zero for DynamicModint<I> {
    fn zero() -> Self {
        Self::new(0)
    }
}

impl<I: Id> One for DynamicModint<I> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<I: Id> Numeric for DynamicModint<I> {}

impl<I: Id> Default for DynamicModint<I> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<I: Id> std::fmt::Debug for DynamicModint<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<I: Id> std::fmt::Display for DynamicModint<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl<I: Id> From<$type> for DynamicModint<I> {
                fn from(value: $type) -> Self {
                    Self::new(value as i64)
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
