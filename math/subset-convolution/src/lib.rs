use zeta_mobius::{subset_mobius, subset_zeta};

pub fn subset_convolution<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::Mul<Output = T>,
{
    let n = a.len();
    assert_eq!(a.len(), b.len());
    assert!(n.is_power_of_two());
    let log_n = n.trailing_zeros() as usize;

    let mut ra = vec![vec![T::default(); n]; log_n + 1];
    let mut rb = vec![vec![T::default(); n]; log_n + 1];
    let mut rc = vec![vec![T::default(); n]; log_n + 1];
    for i in 0..n {
        let p = i.count_ones() as usize;
        ra[p][i] = a[i];
        rb[p][i] = b[i];
    }

    // Subset zeta
    for i in 0..=log_n {
        subset_zeta(&mut ra[i]);
        subset_zeta(&mut rb[i]);
    }

    for i in 0..=log_n {
        for j in 0..=i {
            let q = i - j;
            for k in 0..n {
                rc[i][k] += ra[j][k] * rb[q][k];
            }
        }
    }

    // Subset mobius
    for i in 0..=log_n {
        subset_mobius(&mut rc[i]);
    }

    let mut c = vec![T::default(); n];
    for i in 0..n {
        let p = i.count_ones() as usize;
        c[i] = rc[p][i];
    }
    c
}
