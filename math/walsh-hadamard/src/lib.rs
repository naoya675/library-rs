pub fn walsh_hadamard<T>(a: &mut [T])
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    let mut i = 1;
    while i < n {
        for j in 0..n {
            if j & i == 0 {
                let x = a[j];
                let y = a[j | i];
                a[j] = x + y;
                a[j | i] = x - y;
            }
        }
        i <<= 1;
    }
}

pub fn walsh_hadamard_inv<T>(a: &mut [T])
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Div<Output = T> + From<u64>,
{
    walsh_hadamard(a);
    let n = T::from(a.len() as u64);
    for a in a.iter_mut() {
        *a = *a / n;
    }
}
