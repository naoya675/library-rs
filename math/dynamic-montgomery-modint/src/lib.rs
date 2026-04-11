use std::sync::atomic::{AtomicU64, Ordering};

pub struct MontgomeryReduction {
    modulus: AtomicU64,
    inv_m: AtomicU64,
    r2: AtomicU64,
}

impl MontgomeryReduction {
    pub const fn new(m: u64) -> Self {
        Self {
            modulus: AtomicU64::new(m),
            inv_m: AtomicU64::new(0),
            r2: AtomicU64::new(0),
        }
    }

    pub fn set(&self, m: u64) {
        assert!(m & 1 == 1);
        self.modulus.store(m, Ordering::Relaxed);
        self.inv_m.store(Self::get_inv_m(m), Ordering::Relaxed);
        self.r2.store(Self::get_r2(m), Ordering::Relaxed);
    }

    pub fn modulus(&self) -> u64 {
        self.modulus.load(Ordering::Relaxed)
    }

    pub fn inv_m(&self) -> u64 {
        self.inv_m.load(Ordering::Relaxed)
    }

    pub fn r2(&self) -> u64 {
        self.r2.load(Ordering::Relaxed)
    }

    fn get_inv_m(m: u64) -> u64 {
        let mut r = m;
        for _ in 0..5 {
            r = r.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(r)));
        }
        r
    }

    fn get_r2(m: u64) -> u64 {
        ((m as u128).wrapping_neg() % m as u128) as u64
    }
}

pub trait Id {
    fn montgomery() -> &'static MontgomeryReduction;
}

pub struct DefaultId;

impl Id for DefaultId {
    fn montgomery() -> &'static MontgomeryReduction {
        static MR: MontgomeryReduction = MontgomeryReduction::new(998244353);
        &MR
    }
}

pub struct DynamicMontgomeryModint<I: Id> {
    value: u64, // Montgomery form (a * R mod M), R = 2^64
    _phantom: std::marker::PhantomData<fn() -> I>,
}

impl<I: Id> Clone for DynamicMontgomeryModint<I> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> Copy for DynamicMontgomeryModint<I> {}

impl<I: Id> PartialEq for DynamicMontgomeryModint<I> {
    fn eq(&self, other: &Self) -> bool {
        let m = Self::get_mod();
        let a = if self.value >= m { self.value - m } else { self.value };
        let b = if other.value >= m { other.value - m } else { other.value };
        a == b
        // self.value() == other.value()
    }
}

impl<I: Id> Eq for DynamicMontgomeryModint<I> {}

impl<I: Id> DynamicMontgomeryModint<I> {
    pub fn set_mod(m: u64) {
        I::montgomery().set(m);
    }

    pub fn get_mod() -> u64 {
        I::montgomery().modulus()
    }

    fn reduce(v: u128) -> u64 {
        let mr = I::montgomery();
        let r = (v as u64).wrapping_mul(mr.inv_m().wrapping_neg());
        ((v + r as u128 * mr.modulus() as u128) >> 64) as u64
    }

    pub fn new(n: i64) -> Self {
        let v = n.rem_euclid(Self::get_mod() as i64) as u64;
        Self {
            value: Self::reduce(v as u128 * I::montgomery().r2() as u128),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn value(&self) -> u64 {
        let res = Self::reduce(self.value as u128);
        if res >= Self::get_mod() { res - Self::get_mod() } else { res }
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
        let (x, _, _) = Self::ext_gcd(self.value() as i64, Self::get_mod() as i64);
        Self::new(x as i64)
    }
}

impl<I: Id> std::ops::Add for DynamicMontgomeryModint<I> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut value = self.value + rhs.value;
        if value >= 2 * Self::get_mod() {
            value -= 2 * Self::get_mod();
        }
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> std::ops::AddAssign for DynamicMontgomeryModint<I> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<I: Id> std::ops::Sub for DynamicMontgomeryModint<I> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut value = 2 * Self::get_mod() + self.value - rhs.value;
        if value >= 2 * Self::get_mod() {
            value -= 2 * Self::get_mod();
        }
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> std::ops::SubAssign for DynamicMontgomeryModint<I> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<I: Id> std::ops::Mul for DynamicMontgomeryModint<I> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: Self::reduce(self.value as u128 * rhs.value as u128),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: Id> std::ops::MulAssign for DynamicMontgomeryModint<I> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<I: Id> std::ops::Div for DynamicMontgomeryModint<I> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(rhs.value() != 0);
        self * rhs.inv()
    }
}

impl<I: Id> std::ops::DivAssign for DynamicMontgomeryModint<I> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<I: Id> std::ops::Neg for DynamicMontgomeryModint<I> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

use numeric_traits::{Numeric, One, Zero};

impl<I: Id> Zero for DynamicMontgomeryModint<I> {
    fn zero() -> Self {
        Self::new(0)
    }
}

impl<I: Id> One for DynamicMontgomeryModint<I> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<I: Id> Numeric for DynamicMontgomeryModint<I> {}

impl<I: Id> Default for DynamicMontgomeryModint<I> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<I: Id> std::fmt::Debug for DynamicMontgomeryModint<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<I: Id> std::fmt::Display for DynamicMontgomeryModint<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl<I: Id> From<$type> for DynamicMontgomeryModint<I> {
                fn from(value: $type) -> Self {
                    Self::new(value as i64)
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
