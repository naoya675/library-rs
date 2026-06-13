#[derive(Debug, Clone)]
pub struct EratosthenesFactorization {
    is_prime: Vec<bool>,
    lpf: Vec<usize>,
    lpf_e: Vec<(usize, u32)>,
    primes: Vec<usize>,
}

impl EratosthenesFactorization {
    pub fn new(n: usize) -> Self {
        let mut is_prime = vec![true; n + 1];
        let mut lpf = vec![1; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=n {
            if is_prime[i] {
                lpf[i] = i;
                if i <= n / i {
                    let mut j = i * i;
                    while j <= n {
                        is_prime[j] = false;
                        if lpf[j] == 1 {
                            lpf[j] = i;
                        }
                        j += i;
                    }
                }
            }
        }
        let mut lpf_e = vec![(1, 0); n + 1];
        for i in 2..=n {
            let p = lpf[i];
            let j = i / p;
            lpf_e[i] = if lpf[j] == p { (lpf_e[j].0 * p, lpf_e[j].1 + 1) } else { (lpf[i], 1) };
        }
        let primes = (2..=n).filter(|&i| is_prime[i]).collect();
        Self { is_prime, lpf, lpf_e, primes }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n]
    }

    pub fn least_factor(&self, n: usize) -> Option<usize> {
        if n < 2 { None } else { Some(self.lpf[n]) }
    }

    pub fn factors_dup(&self, mut n: usize) -> Vec<usize> {
        let mut res = vec![];
        while n > 1 {
            res.push(self.lpf[n]);
            n /= self.lpf[n];
        }
        res
    }

    pub fn factorize(&self, mut n: usize) -> Vec<(usize, u32)> {
        let mut res = vec![];
        while n > 1 {
            res.push((self.lpf[n], self.lpf_e[n].1));
            n /= self.lpf_e[n].0;
        }
        res
    }

    pub fn euler_phi(&self, n: usize) -> usize {
        let mut res = n;
        for (p, _) in self.factorize(n) {
            res = res / p * (p - 1);
        }
        res
    }

    pub fn mobius(&self, n: usize) -> i32 {
        let mut res = 1;
        for (_, e) in self.factorize(n) {
            if e >= 2 {
                return 0;
            }
            res = -res;
        }
        res
    }

    pub fn divisors(&self, n: usize) -> Vec<usize> {
        let mut res = vec![1];
        for (p, e) in self.factorize(n) {
            let mut tmp = vec![];
            let mut pp = 1;
            for _ in 1..=e {
                pp *= p;
                tmp.extend(res.iter().map(|&x| x * pp));
            }
            res.extend(tmp);
        }
        res.sort_unstable();
        res
    }

    pub fn divisors_count(&self, n: usize) -> usize {
        self.factorize(n).iter().map(|&(_, e)| e as usize + 1).product()
    }

    pub fn primes(&self) -> &[usize] {
        &self.primes
    }
}
