#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModintMersenne61 {
    value: u64,
}

impl ModintMersenne61 {
    const M: u64 = (1u64 << 61) - 1;

    pub fn new(n: i64) -> Self {
        Self {
            value: n.rem_euclid(Self::M as i64) as u64,
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
        let (x, _, _) = Self::ext_gcd(self.value() as i64, Self::M as i64);
        Self { value: x as u64 }
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

    fn mod_mersenne(v: u64) -> u64 {
        let vu = v >> 61;
        let vd = v & ((1u64 << 61) - 1);
        let su = vu + vd;
        if su >= Self::M { su - Self::M } else { su }
    }

    fn mul_mersenne(lhs: u64, rhs: u64) -> u64 {
        let lu = lhs >> 31;
        let ld = lhs & ((1u64 << 31) - 1);
        let ru = rhs >> 31;
        let rd = rhs & ((1u64 << 31) - 1);
        let mid = ld * ru + lu * rd;
        let mu = mid >> 30;
        let md = mid & ((1u64 << 30) - 1);
        let su = ((lu * ru) << 1) + mu + (md << 31) + ld * rd;
        Self::mod_mersenne(su)
    }

    pub fn rand() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::new(rng.random_range(1..Self::M as i64))

        // rand = "0.8.5"
        // let mut rng = rand::thread_rng();
        // Self::new(rng.gen_range(1..Self::M as i64))
    }
}

impl std::ops::Add for ModintMersenne61 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut value = self.value + rhs.value;
        if value >= Self::M {
            value -= Self::M;
        }
        Self { value }
    }
}

impl std::ops::AddAssign for ModintMersenne61 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for ModintMersenne61 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut value = Self::M + self.value - rhs.value;
        if value >= Self::M {
            value -= Self::M;
        }
        Self { value }
    }
}

impl std::ops::SubAssign for ModintMersenne61 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for ModintMersenne61 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: Self::mul_mersenne(self.value, rhs.value),
        }
    }
}

impl std::ops::MulAssign for ModintMersenne61 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for ModintMersenne61 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(rhs.value != 0);
        self * rhs.inv()
    }
}

impl std::ops::DivAssign for ModintMersenne61 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for ModintMersenne61 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

use numeric_traits::{Numeric, One, Zero};

impl Zero for ModintMersenne61 {
    fn zero() -> Self {
        Self::new(0)
    }
}

impl One for ModintMersenne61 {
    fn one() -> Self {
        Self::new(1)
    }
}

impl Numeric for ModintMersenne61 {}

impl Default for ModintMersenne61 {
    fn default() -> Self {
        Self::new(0)
    }
}

impl std::fmt::Debug for ModintMersenne61 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for ModintMersenne61 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl From<$type> for ModintMersenne61 {
                fn from(value: $type) -> Self {
                    Self::new(value as i64)
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
