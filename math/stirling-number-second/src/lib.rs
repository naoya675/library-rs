use binomial::Binomial;

pub fn stirling_number_second<T>(n: usize, k: usize) -> T
where
    T: Copy + From<u64>,
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::SubAssign,
    T: std::ops::Mul<T, Output = T>,
    T: std::ops::MulAssign,
    T: std::ops::Div<T, Output = T>,
    T: std::ops::DivAssign,
{
    let mut bi = Binomial::<T>::new();
    let mut res = T::from(0);
    for i in 0..=k {
        let term = pow(T::from(i as u64), n) * bi.comb(k, i);
        if (k - i) % 2 == 0 {
            res += term;
        } else {
            res -= term;
        }
    }
    res * bi.finv(k)
}

fn pow<T>(mut p: T, mut e: usize) -> T
where
    T: Copy + From<u64>,
    T: std::ops::Mul<T, Output = T>,
    T: std::ops::MulAssign,
{
    let mut res = T::from(1);
    while e > 0 {
        if e & 1 == 1 {
            res = res * p;
        }
        p = p * p;
        e >>= 1;
    }
    res
}
