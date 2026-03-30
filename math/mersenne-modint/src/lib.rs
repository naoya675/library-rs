#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MersenneModint {
    value: u64,
}

impl MersenneModint {
    const MOD: u64 = (1u64 << 61) - 1;
    const MASK30: u64 = (1u64 << 30) - 1;
    const MASK31: u64 = (1u64 << 31) - 1;
    const MASK61: u64 = Self::MOD;

    pub fn new(n: i64) -> Self {
        Self {
            value: n.rem_euclid(Self::MOD as i64) as u64,
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
        let (x, _, _) = Self::ext_gcd(self.value() as i64, Self::MOD as i64);
        Self { value: x as u64 }
    }

    pub fn rand() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::new(rng.random_range(Self::MASK31 as i64..Self::MASK61 as i64))

        // rand = "0.8.5"
        // let mut rng = rand::thread_rng();
        // Self::new(rng.gen_range(Self::MASK31 as i64..Self::MASK61 as i64))
    }
}

impl std::ops::Add for MersenneModint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut value = self.value + rhs.value;
        if value >= Self::MOD {
            value -= Self::MOD;
        }
        Self { value }
    }
}

impl std::ops::AddAssign for MersenneModint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for MersenneModint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut value = Self::MOD + self.value - rhs.value;
        if value >= Self::MOD {
            value -= Self::MOD;
        }
        Self { value }
    }
}

impl std::ops::SubAssign for MersenneModint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for MersenneModint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let au = self.value >> 31;
        let ad = self.value & Self::MASK31;
        let bu = rhs.value() >> 31;
        let bd = rhs.value() & Self::MASK31;
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & Self::MASK30;
        let su = ((au * bu) << 1) + midu + (midd << 31) + ad * bd;
        Self {
            value: (su >> 61) + (su & Self::MASK61),
        }
    }
}

impl std::ops::MulAssign for MersenneModint {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for MersenneModint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.value == 0 {
            panic!();
        }
        self * rhs.inv()
    }
}

impl std::ops::DivAssign for MersenneModint {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for MersenneModint {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for MersenneModint {
    fn zero() -> Self {
        Self::new(0)
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for MersenneModint {
    fn one() -> Self {
        Self::new(1)
    }
}

impl Default for MersenneModint {
    fn default() -> Self {
        Self::new(0)
    }
}

impl std::fmt::Display for MersenneModint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl From<$type> for MersenneModint {
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
        impl std::ops::$trait for MersenneModint {
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
