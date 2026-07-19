use walsh_hadamard::{walsh_hadamard, walsh_hadamard_inv};

pub fn xor_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + From<u64>,
{
    assert_eq!(a.len(), b.len());
    walsh_hadamard(&mut a);
    walsh_hadamard(&mut b);
    let mut c: Vec<T> = (0..a.len()).map(|i| a[i] * b[i]).collect();
    walsh_hadamard_inv(&mut c);
    c
}
