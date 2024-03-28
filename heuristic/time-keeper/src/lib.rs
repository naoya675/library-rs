#[derive(Debug, Clone)]
struct TimeKeeper {
    time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            time: std::time::Instant::now(),
            time_threshold,
        }
    }

    fn is_time_over(&self) -> bool {
        let elapsed_time = self.time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.90 >= self.time_shreshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
}
