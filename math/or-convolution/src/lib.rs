use zeta_mobius::{subset_mobius, subset_zeta};

pub fn or_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    subset_zeta(&mut a);
    subset_zeta(&mut b);
    let mut c: Vec<T> = (0..a.len()).map(|i| a[i] * b[i]).collect();
    subset_mobius(&mut c);
    c
}
