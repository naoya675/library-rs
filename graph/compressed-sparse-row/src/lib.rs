#[derive(Debug, Clone)]
pub struct Csr<E> {
    pub start: Vec<usize>,
    pub elist: Vec<E>,
}

impl<E: Copy> Csr<E> {
    pub fn new(n: usize, edges: &[(usize, E)]) -> Self {
        let mut start = vec![0; n + 1];
        for &(from, _) in edges {
            start[from + 1] += 1;
        }
        for i in 0..n {
            start[i + 1] += start[i];
        }
        let mut counter = start.clone();
        let mut perm = vec![0; edges.len()];
        for (i, &(from, _)) in edges.iter().enumerate() {
            perm[counter[from]] = i;
            counter[from] += 1;
        }
        let elist = perm.into_iter().map(|i| edges[i].1).collect();
        Self { start, elist }
    }
}

impl<E> std::ops::Index<usize> for Csr<E> {
    type Output = [E];
    fn index(&self, v: usize) -> &[E] {
        &self.elist[self.start[v]..self.start[v + 1]]
    }
}
