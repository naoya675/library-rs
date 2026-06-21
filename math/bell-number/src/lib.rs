use binomial::Binomial;

pub fn bell_number<T>(n: usize, k: usize) -> T
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
    let mut pre = vec![T::from(0); k + 1];
    let mut acc = T::from(0);
    for i in 0..=k {
        if i % 2 == 0 {
            acc += bi.finv(i);
        } else {
            acc -= bi.finv(i);
        }
        pre[i] = acc;
    }
    let mut res = T::from(0);
    for i in 0..=k {
        res += pow(T::from(i as u64), n) * bi.finv(i) * pre[k - i];
    }
    res
}

fn pow<T>(mut p: T, mut e: usize) -> T
where
    T: Copy + From<u64> + std::ops::Mul<T, Output = T>,
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
