pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    (r0, s0, t0)
}

pub fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let (g, x, _) = ext_gcd(a, b);
    (g, x.rem_euclid(b / g))
}

pub fn inv_mod(a: i64, m: i64) -> Option<i64> {
    let (g, x) = inv_gcd(a, m);
    if g == 1 { Some(x) } else { None }
}
