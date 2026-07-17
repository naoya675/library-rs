pub fn subset_zeta<T: Copy + std::ops::AddAssign>(a: &mut [T]) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let log_n = n.trailing_zeros() as usize;
    for i in 0..log_n {
        let bit = 1 << i;
        for j in 0..n {
            if j & bit != 0 {
                let t = a[j ^ bit];
                a[j] += t;
            }
        }
    }
}

pub fn subset_mobius<T: Copy + std::ops::SubAssign>(a: &mut [T]) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let log_n = n.trailing_zeros() as usize;
    for i in 0..log_n {
        let bit = 1 << i;
        for j in 0..n {
            if j & bit != 0 {
                let t = a[j ^ bit];
                a[j] -= t;
            }
        }
    }
}

pub fn superset_zeta<T: Copy + std::ops::AddAssign>(a: &mut [T]) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let log_n = n.trailing_zeros() as usize;
    for i in 0..log_n {
        let bit = 1 << i;
        for j in 0..n {
            if j & bit == 0 {
                let t = a[j | bit];
                a[j] += t;
            }
        }
    }
}

pub fn superset_mobius<T: Copy + std::ops::SubAssign>(a: &mut [T]) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let log_n = n.trailing_zeros() as usize;
    for i in 0..log_n {
        let bit = 1 << i;
        for j in 0..n {
            if j & bit == 0 {
                let t = a[j | bit];
                a[j] -= t;
            }
        }
    }
}
