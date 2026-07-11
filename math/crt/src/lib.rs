use ext_gcd::inv_gcd;

pub fn crt(r: &[i64], m: &[i64]) -> Option<(i64, i64)> {
    assert_eq!(r.len(), m.len());
    let mut r0 = 0;
    let mut m0 = 1;
    for i in 0..r.len() {
        assert!(m[i] >= 1);
        let mut r1 = r[i].rem_euclid(m[i]);
        let mut m1 = m[i];
        if m0 < m1 {
            std::mem::swap(&mut r0, &mut r1);
            std::mem::swap(&mut m0, &mut m1);
        }
        if m0 % m1 == 0 {
            if r0 % m1 != r1 {
                return None;
            }
            continue;
        }
        let (g, im) = inv_gcd(m0, m1);
        if (r1 - r0) % g != 0 {
            return None;
        }
        let u1 = m1 / g;
        let k = (r1 - r0) / g % u1 * im % u1;
        r0 += k * m0;
        m0 *= u1;
        if r0 < 0 {
            r0 += m0;
        }
    }
    Some((r0, m0))
}
