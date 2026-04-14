pub trait Join: Iterator {
    fn join(self, sep: &str) -> String
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        let mut s = String::new();
        for (i, item) in self.enumerate() {
            if i > 0 {
                s.push_str(sep);
            }
            s.push_str(&item.to_string());
        }
        s
    }
}

impl<I: Iterator> Join for I {}
