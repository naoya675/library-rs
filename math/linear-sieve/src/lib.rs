#[derive(Debug, Clone)]
pub struct LinearSieve {
    lpf: Vec<usize>,
    lpf_e: Vec<(usize, u32)>,
    primes: Vec<usize>,
}

impl LinearSieve {
    pub fn new(n: usize) -> Self {
        let mut lpf = vec![1; n + 1];
        let mut primes = vec![];
        for i in 2..=n {
            if lpf[i] == 1 {
                lpf[i] = i;
                primes.push(i);
            }
            let lpf_i = lpf[i];
            for &j in primes.iter().take_while(|&&j| j <= lpf_i.min(n / i)) {
                lpf[i * j] = j;
            }
        }
        let mut lpf_e = vec![(1, 0); n + 1];
        for i in 2..=n {
            let p = lpf[i];
            let j = i / p;
            lpf_e[i] = if lpf[j] == p { (lpf_e[j].0 * p, lpf_e[j].1 + 1) } else { (lpf[i], 1) };
        }
        Self { lpf, lpf_e, primes }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        n >= 2 && self.lpf[n] == n
    }

    pub fn least_factor(&self, n: usize) -> Option<usize> {
        if n < 2 {
            None
        } else {
            Some(self.lpf[n])
        }
    }

    pub fn factors_dup(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf[n]))
            .take_while(|&n| n > 1)
            .map(move |n| self.lpf[n])
    }

    pub fn factors(&self, n: usize) -> impl Iterator<Item = (usize, u32)> + '_ {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf_e[n].0))
            .take_while(|&n| n > 1)
            .map(move |n| (self.lpf[n], self.lpf_e[n].1))
    }

    pub fn euler_phi(&self, n: usize) -> usize {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf_e[n].0))
            .take_while(|&n| n > 1)
            .map(|n| self.lpf_e[n].0 / self.lpf[n] * (self.lpf[n] - 1))
            .product()
    }

    pub fn divisors(&self, n: usize) -> impl Iterator<Item = usize> + DoubleEndedIterator {
        let mut res = vec![1];
        for (p, e) in self.factors(n) {
            let mut tmp = vec![];
            let mut pp = 1;
            for _ in 1..=e {
                pp *= p;
                tmp.extend(res.iter().map(|&x| x * pp));
            }
            res.extend(tmp);
        }
        res.sort_unstable();
        res.into_iter()
    }

    pub fn divisors_count(&self, n: usize) -> usize {
        self.factors(n).map(|(_, e)| e as usize + 1).product()
    }

    pub fn primes(&self) -> impl Iterator<Item = usize> + DoubleEndedIterator + '_ {
        self.primes.iter().copied()
    }
}
