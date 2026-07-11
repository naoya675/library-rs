use ext_gcd::inv_gcd;

pub fn garner(r: &[i64], m: &[i64], modulus: i64) -> i64 {
    assert_eq!(r.len(), m.len());
    let mut m = m.to_vec();
    m.push(modulus);
    let n = m.len();
    let mut coeffs = vec![1; n];
    let mut consts = vec![0; n];
    for k in 0..n - 1 {
        let (_, inv) = inv_gcd(coeffs[k], m[k]);
        let t = ((r[k] - consts[k]) * inv).rem_euclid(m[k]);
        for i in k + 1..n {
            consts[i] = (consts[i] + t * coeffs[i]) % m[i];
            coeffs[i] = (coeffs[i] * m[k]) % m[i];
        }
    }
    consts[n - 1]
}
