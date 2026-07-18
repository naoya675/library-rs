use zeta_mobius::{superset_mobius, superset_zeta};

pub fn and_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    superset_zeta(&mut a);
    superset_zeta(&mut b);
    let mut c: Vec<T> = (0..a.len()).map(|i| a[i] * b[i]).collect();
    superset_mobius(&mut c);
    c
}
