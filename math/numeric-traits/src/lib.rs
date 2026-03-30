use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Numeric:
    Copy
    + PartialEq
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
}

/*
impl<T> Numeric for T where
    T: Copy
        + PartialEq
        + Zero
        + One
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
{
}
*/

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_numeric_traits {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                fn zero() -> Self { 0 }
            }
            impl One for $t {
                fn one() -> Self { 1 }
            }
            impl BoundedBelow for $t {
                fn min_value() -> Self { Self::MIN }
            }
            impl BoundedAbove for $t {
                fn max_value() -> Self { Self::MAX }
            }
            impl Numeric for $t {}
        )*
    };
}

impl_numeric_traits!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
