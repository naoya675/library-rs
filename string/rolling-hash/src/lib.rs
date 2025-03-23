use rand::Rng;

#[derive(Debug, Clone)]
pub struct RollingHash {
    base: u64,
    power: Vec<u64>,
}

impl RollingHash {
    const MOD: u64 = (1_u64 << 61) - 1;
    const MASK30: u64 = (1_u64 << 30) - 1;
    const MASK31: u64 = (1_u64 << 31) - 1;
    const MASK61: u64 = Self::MOD;
    const POSITIVIZER: u64 = Self::MOD * 4;

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let base = rng.gen_range(Self::MASK31..Self::MASK61);
        Self {
            base,
            power: vec![1],
        }
    }

    pub fn build(&mut self, s: &Vec<char>) -> Vec<u64> {
        let size = s.len();
        let mut hash = vec![0; size + 1];
        for i in 0..size {
            hash[i + 1] = Self::calc_mod(Self::calc_mul(hash[i], self.base) + s[i] as u64);
        }
        hash
    }

    fn build_power(&mut self, r: usize) {
        while self.power.len() <= r {
            let val = *self.power.last().unwrap();
            self.power
                .push(Self::calc_mod(Self::calc_mul(val, self.base)));
        }
    }

    // [l, r)
    pub fn rolling_hash(&mut self, hash: &Vec<u64>, l: usize, r: usize) -> u64 {
        assert!(l <= r && r <= hash.len());
        self.build_power(r - l);
        Self::calc_mod(hash[r] + Self::POSITIVIZER - Self::calc_mul(hash[l], self.power[r - l]))
    }

    /*
    fn calc_add(a: u64, b: u64) -> u64 {
        let mut res = a + b;
        if res >= Self::MOD {
            res -= Self::MOD;
        }
        res
    }
    */

    fn calc_mul(a: u64, b: u64) -> u64 {
        let au = a >> 31;
        let ad = a & Self::MASK31;
        let bu = b >> 31;
        let bd = b & Self::MASK31;
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & Self::MASK30;
        ((au * bu) << 1) + midu + (midd << 31) + ad * bd
        // Self::calc_mod(((au * bu) << 1) + midu + (midd << 31) + ad * bd)
    }

    fn calc_mod(a: u64) -> u64 {
        let au = a >> 61;
        let ad = a & Self::MASK61;
        let mut res = au + ad;
        if res >= Self::MOD {
            res -= Self::MOD;
        }
        res
        // Self::calc_add(au, ad)
    }
}
