pub fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=n).filter(|&i| is_prime[i]).collect()
}

/*
pub fn eratosthenes(n: usize) -> Vec<usize> {
    let mut lpf = vec![1; n + 1];
    let mut i = 2;
    while i <= n {
        if lpf[i] == 0 {
            lpf[i] = i;
            if i <= n / i {
                let mut j = i * i;
                while j <= n {
                    if lpf[j] == 1 {
                        lpf[j] = i;
                    }
                    j += i;
                }
            }
        }
        i += 1;
    }
    lpf
}
 */
