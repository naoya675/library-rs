use divisor_zeta_mobius::{divisor_mobius, divisor_zeta};

pub fn lcm_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    divisor_zeta(&mut a);
    divisor_zeta(&mut b);
    let mut c: Vec<T> = (0..a.len()).map(|i| a[i] * b[i]).collect();
    divisor_mobius(&mut c);
    c
}
