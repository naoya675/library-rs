pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    assert!(0 <= n);
    assert!(1 <= m);
    let mut ans: i64 = 0;
    if a < 0 {
        let a2 = a.rem_euclid(m);
        ans = ans.wrapping_sub(n * (n - 1) / 2 * ((a2 - a) / m));
        a = a2;
    }
    if b < 0 {
        let b2 = b.rem_euclid(m);
        ans = ans.wrapping_sub(n * ((b2 - b) / m));
        b = b2;
    }
    ans.wrapping_add(floor_sum_unsigned(n, m, a, b))
}

fn floor_sum_unsigned(mut n: i64, mut m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans: i64 = 0;
    loop {
        if a >= m {
            ans = ans.wrapping_add(n * (n - 1) / 2 * (a / m));
            a %= m;
        }
        if b >= m {
            ans = ans.wrapping_add(n * (b / m));
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    ans
}
