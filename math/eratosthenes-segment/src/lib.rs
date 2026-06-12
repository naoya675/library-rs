pub fn eratosthenes_segment(l: usize, r: usize) -> Vec<usize> {
    assert!(l < r);
    let max = r.isqrt();
    let mut base_is_prime = vec![true; max + 1];
    base_is_prime[0] = false;
    base_is_prime[1] = false;
    let mut is_prime = vec![true; r - l];
    let mut i = 2;
    while i * i <= r {
        if base_is_prime[i] {
            let mut j = i * i;
            while j <= max {
                base_is_prime[j] = false;
                j += i;
            }
            let mut j = (i * i).max((l + i - 1) / i * i);
            while j < r {
                is_prime[j - l] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2.max(l)..r).filter(|&i| is_prime[i - l]).collect()
}
