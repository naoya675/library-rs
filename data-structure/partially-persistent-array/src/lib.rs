#[derive(Debug, Clone)]
pub struct PartiallyPersistentArray<T: Clone> {
    history: Vec<Vec<(usize, T)>>,
    now: usize,
}

impl<T: Clone + Default> PartiallyPersistentArray<T> {
    pub fn new(n: usize) -> Self {
        let history = (0..n).map(|_| vec![(0, T::default())]).collect();
        Self { history, now: 0 }
    }
}

impl<T: Clone> PartiallyPersistentArray<T> {
    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn set(&mut self, i: usize, x: T) {
        assert!(i < self.history.len());
        self.now += 1;
        self.history[i].push((self.now, x));
    }

    pub fn get(&self, t: usize, i: usize) -> T {
        assert!(i < self.history.len());
        let k = self.history[i].partition_point(|&(tt, _)| tt <= t);
        self.history[i][k - 1].1.clone()
    }

    pub fn now(&self) -> usize {
        self.now
    }
}

impl<T: Clone> From<Vec<T>> for PartiallyPersistentArray<T> {
    fn from(init: Vec<T>) -> Self {
        let history = init.into_iter().map(|v| vec![(0, v)]).collect();
        Self { history, now: 0 }
    }
}
