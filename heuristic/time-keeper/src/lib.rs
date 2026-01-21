#[derive(Debug, Clone)]
pub struct TimeKeeper {
    time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    pub fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            time: std::time::Instant::now(),
            time_threshold,
        }
    }

    pub fn elapsed_time(&self) -> f64 {
        let elapsed_time = self.time.elapsed().as_nanos() as f64 * 1e-9;
        elapsed_time
    }

    pub fn elapsed_ratio(&self) -> f64 {
        let elapsed_time = self.time.elapsed().as_nanos() as f64 * 1e-9;
        elapsed_time / self.time_threshold
    }

    pub fn is_time_over(&self) -> bool {
        let elapsed_time = self.time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.90 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time * 0.90 >= self.time_threshold
        }
    }
}
