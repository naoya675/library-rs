pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_zero {
    ($($type:ty), *) => {
        $(
            impl Zero for $type {
                fn zero() -> Self {
                    0
                }
                fn is_zero(&self) -> bool {
                    *self == 0
                }
            }
        )*
    };
}

macro_rules! impl_one {
    ($($type:ty), *) => {
        $(
            impl One for $type {
                fn one() -> Self {
                    1
                }
                fn is_one(&self) -> bool {
                    *self == 1
                }
            }
        )*
    };
}

macro_rules! impl_bounded_below {
    ($($type:ty), *) => {
        $(
            impl BoundedBelow for $type {
                fn min_value() -> Self {
                    Self::min_value()
                }
            }
        )*
    };
}

macro_rules! impl_bounded_above {
    ($($type:ty), *) => {
        $(
            impl BoundedAbove for $type {
                fn max_value() -> Self {
                    Self::max_value()
                }
            }
        )*
    };
}

impl_zero!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);
impl_one!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);
impl_bounded_below!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);
impl_bounded_above!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);
