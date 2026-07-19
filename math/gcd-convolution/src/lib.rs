use divisor_zeta_mobius::{multiple_mobius, multiple_zeta};

pub fn gcd_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    multiple_zeta(&mut a);
    multiple_zeta(&mut b);
    let mut c: Vec<T> = (0..a.len()).map(|i| a[i] * b[i]).collect();
    multiple_mobius(&mut c);
    c
}
