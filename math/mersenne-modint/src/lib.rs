#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MersenneModint {
    value: u64,
}

impl MersenneModint {
    const MOD: u64 = (1_u64 << 61) - 1;
    const MASK30: u64 = (1_u64 << 30) - 1;
    const MASK31: u64 = (1_u64 << 31) - 1;
    const MASK61: u64 = Self::MOD;

    pub fn new(n: u64) -> Self {
        Self {
            value: (n % Self::MOD),
            // value: (n.rem_euclid(Self::MOD)),
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
        self.pow(Self::MOD - 2)
    }

    pub fn rand() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(Self::MASK31..Self::MASK61))
    }
}

impl std::ops::Add for MersenneModint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: (self.value + rhs.value) % Self::MOD,
        }
    }
}

impl std::ops::AddAssign for MersenneModint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for MersenneModint {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        if self.value < rhs.value {
            self.value += Self::MOD;
        }
        Self {
            value: (self.value - rhs.value) % Self::MOD,
        }
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

impl num_traits::Zero for MersenneModint {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        Self::new(0) == *self
    }
}

impl num_traits::One for MersenneModint {
    fn one() -> Self {
        Self::new(1)
    }
}

impl std::fmt::Display for MersenneModint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<u64> for MersenneModint {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/*
macro_rules! impl_from {
    ($($type:ty), *) => {
        $(
            impl From<$type> for MersenneModint {
                fn from(value: $type) -> Self {
                    Self::new(value as u64)
                }
            }
        )*
    };
}

impl_from!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);
*/

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
