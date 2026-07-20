use time_keeper::TimeKeeper;
use xorshift_64::XorShift64;

pub struct SimulatedAnnealing {
    pub time_limit: f64,
    pub start_temp: f64,
    pub end_temp: f64,
}

impl SimulatedAnnealing {
    pub fn new(time_limit: f64, start_temp: f64, end_temp: f64) -> Self {
        Self {
            time_limit,
            start_temp,
            end_temp,
        }
    }

    pub fn run<S, F>(&self, state: &mut S, rng: &mut XorShift64, mut step: F) -> u64
    where
        F: FnMut(&mut S, &mut XorShift64, f64),
    {
        let tk = TimeKeeper::new(self.time_limit);
        let mut iters = 0;
        while !tk.is_time_over() {
            let temp = self.start_temp + (self.end_temp - self.start_temp) * tk.elapsed_ratio();
            step(state, rng, temp);
            iters += 1;
        }
        iters
    }

    pub fn run_exp<S, F>(&self, state: &mut S, rng: &mut XorShift64, mut step: F) -> u64
    where
        F: FnMut(&mut S, &mut XorShift64, f64),
    {
        let tk = TimeKeeper::new(self.time_limit);
        let base = self.end_temp / self.start_temp;
        let mut iters = 0;
        while !tk.is_time_over() {
            let temp = self.start_temp * base.powf(tk.elapsed_ratio());
            step(state, rng, temp);
            iters += 1;
        }
        iters
    }
}

pub fn accept(delta: f64, temp: f64, rng: &mut XorShift64) -> bool {
    delta >= 0.0 || rng.random_f64() < (delta / temp).exp()
}
