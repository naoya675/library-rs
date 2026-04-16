#[derive(Debug, Clone)]
pub struct Csr<E> {
    pub start: Vec<usize>,
    pub elist: Vec<E>,
}

impl<E: Copy + Default> Csr<E> {
    pub fn new(n: usize, edges: &[(usize, E)]) -> Self {
        let mut start = vec![0; n + 1];
        let mut elist = vec![E::default(); edges.len()];
        for &(from, _) in edges {
            start[from + 1] += 1;
        }
        for i in 1..=n {
            start[i] += start[i - 1];
        }
        let mut counter = start.clone();
        for &(from, e) in edges {
            elist[counter[from]] = e;
            counter[from] += 1;
        }
        Self { start, elist }
    }
}

impl<E> std::ops::Index<usize> for Csr<E> {
    type Output = [E];
    fn index(&self, v: usize) -> &[E] {
        &self.elist[self.start[v]..self.start[v + 1]]
    }
}
